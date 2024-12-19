use std::path::PathBuf;
use std::fs;
use crate::error::AudioPlayerError;
use crate::player::stream::StreamHandler;
use crate::player::byte_parser;

#[derive(PartialEq)]
enum State {
    WaitingForFile,
    Paused,
    Playing,
}

pub struct AudioPlayer {
    state: State,
    stream_handler: Option<StreamHandler>,
    current_file_name: Option<String>,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            state: State::WaitingForFile,
            stream_handler: None,
            current_file_name: None,
        }
    }

    pub fn play_file(&mut self, path: PathBuf) -> Result<(), AudioPlayerError> {
        // Leitura do arquivo e processamento
        self.current_file_name = match path.file_stem() {
            Some(os_str) => match os_str.to_os_string().into_string() {
                Ok(file_name) => Some(file_name),
                Err(_) => return Err(AudioPlayerError::InvalidFileName),
            },
            None => return Err(AudioPlayerError::InvalidFileName),
        };

        let file_bytes = fs::read(&path).map_err(AudioPlayerError::IoError)?;

        let stream_handler = match path.extension().and_then(|ext| ext.to_str()) {
            Some("wav") | Some("wave") => wav::stream_from_wav_file(&file_bytes),
            _ => return Err(AudioPlayerError::UnsupportedFileFormat),
        };

        stream_handler.play();
        self.stream_handler = Some(stream_handler);
        self.state = State::Playing;

        Ok(())
    }

    pub fn track_name(&self) -> &Option<String> {
        &self.current_file_name
    }

    pub fn toggle_playing(&mut self) {
        match self.state {
            State::Playing => {
                self.state = State::Paused;
                if let Some(stream_handler) = &self.stream_handler {
                    stream_handler.pause();
                }
            },
            State::Paused => {
                if let Some(stream_handler) = &self.stream_handler {
                    stream_handler.play();
                }
                self.state = State::Playing;
            },
            _ => {},
        }
    }

    pub fn pause_or_play_button_text(&self) -> &str {
        match self.state {
            State::Playing => "Pause",
            _ => "Play",
        }
    }

    pub fn is_playing(&self) -> bool {
        self.state == State::Playing
    }

    pub fn restart(&self) {
        if let Some(stream_handler) = &self.stream_handler {
            stream_handler.restart();
        }
    }

    pub fn progress(&self) -> f32 {
        if let Some(stream_handler) = &self.stream_handler {
            stream_handler.progress()
        } else {
            0.0
        }
    }
}

