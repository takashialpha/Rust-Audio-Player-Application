use crate::error::error::AudioPlayerError;
use std::fs;
use std::path::Path;
use std::process;

pub struct Cache {
    pid: u32,
    pub cache_dir: String,
}

impl Cache {
    pub fn init() -> Result<Cache, AudioPlayerError> {
        let pid = process::id();
        let cache_dir = format!("/tmp/audium-{}", pid);
        let path = Path::new(&cache_dir);

        if path.exists() {
            return Err(AudioPlayerError::DirectoryAlreadyExists);
        }

        match fs::create_dir_all(&cache_dir) {
            Ok(_) => Ok(Cache { pid, cache_dir }),
            Err(e) => Err(AudioPlayerError::IoError(e)),
        }
    }

    pub fn clean(&self) -> Result<(), AudioPlayerError> {
        match fs::remove_dir_all(&self.cache_dir) {
            Ok(_) => Ok(()),
            Err(e) => Err(AudioPlayerError::IoError(e)),
        }
    }
}
