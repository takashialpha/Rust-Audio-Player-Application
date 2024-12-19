use cpal::{Device, SampleFormat, Stream, StreamConfig};
use cpal::traits::{HostTrait, StreamTrait, DeviceTrait};
use crate::error::error::AudioPlayerError;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct StreamHandler {
    stream: Stream,
    total_samples: usize,
    cursor: Arc<AtomicUsize>,
}

impl StreamHandler {
    pub fn from_samples<T>(samples: Vec<T>) -> Result<Self, AudioPlayerError>
    where
        T: cpal::Sample + Send + 'static, // Ensure T implements cpal::Sample
    {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or(AudioPlayerError::StreamError("No output device available".to_string()))?;
        let supported_config = device.default_output_config().map_err(|_| AudioPlayerError::StreamError("Error querying configs".to_string()))?;
        let config = supported_config.config();

        let total_samples = samples.len();
        let cursor = Arc::new(AtomicUsize::new(0));
        let stream_cursor = Arc::clone(&cursor);

        let stream = match supported_config.sample_format() {
            SampleFormat::I16 => build_stream::<T, i16>(device, config, samples, stream_cursor),
            SampleFormat::U8 => build_stream::<T, u8>(device, config, samples, stream_cursor),
            _ => return Err(AudioPlayerError::StreamError("Unsupported sample format".to_string())),
        };

        stream.play().map_err(|_| AudioPlayerError::StreamError("Failed to start the stream".to_string()))?;

        Ok(Self {
            stream,
            total_samples,
            cursor,
        })
    }

    pub fn play(&self) {
        self.stream.play().expect("Failed to play stream");
    }

    pub fn pause(&self) {
        self.stream.pause().expect("Failed to pause stream");
    }

    // Restart the stream
    pub fn restart(&self) {
        self.stream.pause().expect("Failed to pause stream");
        self.cursor.store(0, Ordering::Relaxed); // Reset the cursor
        self.stream.play().expect("Failed to restart stream");
    }

    pub fn progress(&self) -> f32 {
        self.cursor.load(Ordering::Relaxed) as f32 / self.total_samples as f32
    }
}

fn build_stream<T, O>(
    device: Device,
    config: StreamConfig,
    audio_buffer: Vec<T>,
    cursor: Arc<AtomicUsize>,
) -> Stream
where
    O: cpal::SizedSample + cpal::Sample,
    T: cpal::Sample + Send + 'static, // Ensure T implements cpal::Sample
{
    let err_fn = |err| eprintln!("Error on audio stream: {}", err);

    let write_output = move |data: &mut [O], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            let index = cursor.fetch_add(1, Ordering::Relaxed);
            if index < audio_buffer.len() {
                // Convert sample to f32 if T implements cpal::Sample
                let sample_f32 = audio_buffer[index].to_f32();
                
                *sample = if std::any::TypeId::of::<O>() == std::any::TypeId::of::<i16>() {
                    sample_f32.round() as i16 as O
                } else if std::any::TypeId::of::<O>() == std::any::TypeId::of::<u8>() {
                    (sample_f32.round() as u8) as O
                } else {
                    cpal::Sample::EQUILIBRIUM
                };
            } else {
                *sample = cpal::Sample::EQUILIBRIUM;
            }
        }
    };

    device.build_output_stream(&config, write_output, err_fn, None).expect("Failed to build output stream")
}

