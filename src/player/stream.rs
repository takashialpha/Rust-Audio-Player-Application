use cpal::{Device, SampleFormat, Stream, StreamConfig};
use cpal::traits::{HostTrait, StreamTrait, DeviceTrait};
use dasp_sample::{Sample, FromSample};
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
        T: cpal::Sample + Sample + Send + 'static,
        i16: Sample<Float = T::Float> + FromSample<T>,
        u8: Sample<Float = T::Float> + FromSample<T>,
        f32: Sample<Float = T::Float> + FromSample<T>,
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(AudioPlayerError::StreamError("No output device available".to_string()))?;
        
        let supported_config = device
            .default_output_config()
            .map_err(|_| AudioPlayerError::StreamError("Error querying configs".to_string()))?;
        
        let config = supported_config.config();
        let total_samples = samples.len();
        let cursor = Arc::new(AtomicUsize::new(0));
        let stream_cursor = Arc::clone(&cursor);

        let stream = match supported_config.sample_format() {
            SampleFormat::I16 => build_stream::<T, i16>(device, config, samples, stream_cursor),
            SampleFormat::U8 => build_stream::<T, u8>(device, config, samples, stream_cursor),
            SampleFormat::F32 => build_stream::<T, f32>(device, config, samples, stream_cursor),
            _ => return Err(AudioPlayerError::StreamError("Unsupported sample format".to_string())),
        }?;

        stream.play().map_err(|_| AudioPlayerError::StreamError("Failed to start the stream".to_string()))?;

        Ok(Self {
            stream,
            total_samples,
            cursor,
        })
    }

    pub fn play(&self) -> Result<(), AudioPlayerError> {
        self.stream
            .play()
            .map_err(|_| AudioPlayerError::StreamError("Failed to play stream".to_string()))
    }

    pub fn pause(&self) -> Result<(), AudioPlayerError> {
        self.stream
            .pause()
            .map_err(|_| AudioPlayerError::StreamError("Failed to pause stream".to_string()))
    }

    pub fn restart(&self) -> Result<(), AudioPlayerError> {
        self.pause()?;
        self.cursor.store(0, Ordering::Relaxed);
        self.play()?;
        Ok(())
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
) -> Result<Stream, AudioPlayerError>
where
    T: cpal::Sample + Sample + Send + 'static,
    O: cpal::Sample + cpal::SizedSample + Sample + FromSample<T>,
{
    let err_fn = |err| eprintln!("Error on audio stream: {}", err);
    
    let write_output = move |data: &mut [O], _: &cpal::OutputCallbackInfo| {
        for sample in data.iter_mut() {
            let index = cursor.fetch_add(1, Ordering::Relaxed);
            if index < audio_buffer.len() {
                *sample = O::from_sample(audio_buffer[index]);
            } else {
                *sample = O::EQUILIBRIUM;
            }
        }
    };

    device
        .build_output_stream(&config, write_output, err_fn, None)
        .map_err(|_| AudioPlayerError::StreamError("Failed to build output stream".to_string()))
}
