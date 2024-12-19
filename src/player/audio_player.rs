use std::path::PathBuf;
use std::fs;
use crate::error::error::AudioPlayerError;
use crate::player::stream::StreamHandler;
use crate::player::wav;

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
        self.current_file_name = path.file_stem()
            .and_then(|os_str| os_str.to_os_string().into_string().ok())
            .ok_or(AudioPlayerError::InvalidFileName)?;

        let file_bytes = fs::read(&path).map_err(AudioPlayerError::IoError)?;

        let stream_handler = match path.extension().and_then(|ext| ext.to_str()) {
            Some("wav") | Some("wave") => wav::stream_from_wav_file(&file_bytes)?,
            _ => return Err(AudioPlayerError::UnsupportedFileFormat),
        };

        stream_handler.play();
        self.stream_handler = Some(stream_handler);
        self.state = State::Playing;

        Ok(())
    }

    pub fn toggle_playing(&mut self) {
        if let Some(stream_handler) = &self.stream_handler {
            match self.state {
                State::Playing => {
                    self.state = State::Paused;
                    stream_handler.pause();
                }
                State::Paused => {
                    self.state = State::Playing;
                    stream_handler.play();
                }
                _ => {}
            }
        }
    }

    pub fn progress(&self) -> f32 {
        self.stream_handler.as_ref().map_or(0.0, |s| s.progress())
    }

    pub fn pause_or_play_button_text(&self) -> &str {
        match self.state {
            State::Playing => "Pause",
            _ => "Play",
        }
    }

    pub fn restart(&mut self) {
        if let Some(stream_handler) = &self.stream_handler {
            stream_handler.restart();
        }
        self.state = State::WaitingForFile; // initial state
    }
}
