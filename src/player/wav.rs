use crate::error::error::AudioPlayerError;
use crate::player::byte_parser;

pub fn stream_from_wav_file(file_bytes: &[u8]) -> Result<Vec<i16>, AudioPlayerError> {
    // Simple check to validate if the file is a valid WAV
    if file_bytes.len() < 44 {
        return Err(AudioPlayerError::StreamError(
            "File is too short to be a valid WAV".to_string(),
        ));
    }

    // Check the WAV header (assuming we are looking for 'RIFF' and 'WAVE')
    let header = &file_bytes[0..4];
    if header != b"RIFF" {
        return Err(AudioPlayerError::StreamError(
            "Invalid WAV header".to_string(),
        ));
    }

    // Extract the samples (ignoring the first 44 bytes of the WAV header)
    let samples = byte_parser::to_type_little_endian::<i16>(&file_bytes[44..]);

    Ok(samples)
}
