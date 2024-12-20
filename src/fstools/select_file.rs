use crate::error::error::AudioPlayerError;
use crate::fstools::cache;
use crate::fstools::cache::Cache;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, exit};

pub struct SelectFile {
    pub file_path: String,
}

impl SelectFile {
    pub fn new() -> Result<SelectFile, AudioPlayerError> {
        match cache::Cache::init() {
            Ok(cache) => {
                let cache_dir = &cache.cache_dir;
                let final_txt_dir = format!("{}/input_ranger_file.txt", cache_dir);
                match File::create(final_txt_dir.clone()) {
                    Ok(_) => Ok(SelectFile {
                        file_path: final_txt_dir,
                    }),
                    Err(e) => Err(AudioPlayerError::IoError(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn clear_file(&mut self) -> io::Result<()> {
        let mut file = File::create(&self.file_path)?;
        file.set_len(0)?;
        Ok(())
    }

    pub fn get_file(&mut self) -> Result<SelectFile, AudioPlayerError> {
        let output = Command::new("ranger")
            .arg("-c")
            .arg("--choosefile")
            .arg(self.file_path.clone())
            .arg("--selectfile")
            .arg("~/")
            .output()
            .map_err(|e| AudioPlayerError::IoError(e))?;

        if !output.status.success() {
            return Err(AudioPlayerError::FailedToSelectFile);
        }

        if self.file_path.is_empty() {
            return Err(AudioPlayerError::NoFileSelected);
        }

        self.file_path = fs::read_to_string(&self.file_path)
            .map_err(|e| AudioPlayerError::IoError(e))?;

        Ok(SelectFile {
            file_path: self.file_path.clone(),
        })
    }
}

