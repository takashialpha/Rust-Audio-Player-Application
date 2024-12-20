use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AudioPlayerError {
    IoError(io::Error),
    UnsupportedFileFormat,
    InvalidFileName,
    StreamError(String),
    DirectoryAlreadyExists,
    NoFileSelected,
    FileAlreadyExists,
    FailedToSelectFile,
}

impl fmt::Display for AudioPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AudioPlayerError::IoError(ref err) => write!(f, "I/O Error: {}", err),
            AudioPlayerError::UnsupportedFileFormat => write!(f, "Unsupported file format"),
            AudioPlayerError::InvalidFileName => write!(f, "Invalid file name"),
            AudioPlayerError::StreamError(ref msg) => write!(f, "Stream Error: {}", msg),
            AudioPlayerError::DirectoryAlreadyExists => {
                write!(f, "Error: Directory already exists")
            }
            AudioPlayerError::NoFileSelected => write!(f, "Error: No file selected"),
            AudioPlayerError::FileAlreadyExists => write!(f, "Error: File already exists"),
            AudioPlayerError::FailedToSelectFile => write!(f, "Error: Failed to select file"),
        }
    }
}

impl From<io::Error> for AudioPlayerError {
    fn from(err: io::Error) -> Self {
        AudioPlayerError::IoError(err)
    }
}
