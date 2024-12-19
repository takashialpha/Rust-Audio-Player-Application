use crate::player::byte_parser;

pub fn stream_from_wav_file(file_bytes: &[u8]) -> Vec<i16> {
    // add validation for WAV file
    // assuming that the file is a valid WAV file

    let samples = byte_parser::to_type_little_endian::<i16>(&file_bytes[44..]); // Pulando o cabeçalho do WAV (44 bytes)

    samples
}

