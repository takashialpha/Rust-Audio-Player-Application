use std::io;
use std::fmt;

#[derive(Debug)]
pub enum AudioPlayerError {
    IoError(io::Error),
    UnsupportedFileFormat,
    InvalidFileName,
    StreamError(String),
}

impl fmt::Display for AudioPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AudioPlayerError::IoError(ref err) => write!(f, "I/O Error: {}", err),
            AudioPlayerError::UnsupportedFileFormat => write!(f, "Unsupported file format"),
            AudioPlayerError::InvalidFileName => write!(f, "Invalid file name"),
            AudioPlayerError::StreamError(ref msg) => write!(f, "Stream Error: {}", msg),
        }
    }
}

impl From<io::Error> for AudioPlayerError {
    fn from(err: io::Error) -> Self {
        AudioPlayerError::IoError(err)
    }
}

