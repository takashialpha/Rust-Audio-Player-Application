use cpal::{Device, SampleFormat, Stream, StreamConfig};
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
        T: cpal::Sample + Send + 'static,
        u8: cpal::FromSample<T>,
        u16: cpal::FromSample<T>,
        u32: cpal::FromSample<T>,
        i8: cpal::FromSample<T>,
        i16: cpal::FromSample<T>,
        i32: cpal::FromSample<T>,
        f32: cpal::FromSample<T>,
    {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or(AudioPlayerError::StreamError("No output device available".to_string()))?;

        let mut supported_configs_range = device.supported_output_configs().map_err(|_| AudioPlayerError::StreamError("Error while querying configs".to_string()))?;

        let supported_config = supported_configs_range
            .next()
            .ok_or(AudioPlayerError::StreamError("No supported config".to_string()))?
            .with_max_sample_rate();

        let config = supported_config.config();
        let sample_format = supported_config.sample_format();

        let total_samples = samples.len();
        let cursor = Arc::new(AtomicUsize::new(0));

        let stream_cursor = Arc::clone(&cursor);
        let stream = match sample_format {
            SampleFormat::I16 => build_stream::<T, i16>(device, config, samples, stream_cursor),
            SampleFormat::U8 => build_stream::<T, u8>(device, config, samples, stream_cursor),
            sample_format => return Err(AudioPlayerError::StreamError(format!("Unsupported sample format '{:?}'", sample_format))),
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

    pub fn progress(&self) -> f32 {
        self.cursor.load(Ordering::Relaxed) as f32 / self.total_samples as f32
    }

    pub fn restart(&self) {
        self.cursor.store(0, Ordering::Relaxed);
    }
}

fn build_stream<T, O>(device: Device, config: StreamConfig, audio_buffer: Vec<T>, cursor: Arc<AtomicUsize>) -> Stream
where
    T: cpal::Sample + Send + 'static,
    O: cpal::SizedSample + cpal::FromSample<T>,
{
    let err_fn = |err| eprintln!("Error on audio stream: {}", err);

    let write_output = move |data: &mut [O], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            let index = cursor.fetch_add(1, Ordering::Relaxed);
            if index < audio_buffer.len() {
                *sample = audio_buffer[index].to_sample::<O>();
            } else {
                *sample = cpal::Sample::EQUILIBRIUM;
            }
        }
    };

    device
        .build_output_stream(&config, write_output, err_fn, None)
        .expect("Failed to build output stream")
}

