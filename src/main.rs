mod error;
mod fstools;
mod player;
mod tui;

use player::audio_player::AudioPlayer;
use std::path::PathBuf;
use tui::tui::Tui;

fn main() {
    let mut player = AudioPlayer::new();
    
    match fstools::select_file::SelectFile::new() {
        Ok(mut select_file) => {
            match select_file.get_file() {
                Ok(_) => {
                    let path = PathBuf::from(&select_file.file_path);

                    if let Err(e) = player.play_file(path) {
                        eprintln!("Error playing file: {}", e);
                    }

                    let mut tui = Tui::new(player);

                    if let Err(e) = tui.run() {
                        eprintln!("Error running the TUI: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error selecting file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error initializing SelectFile: {}", e);
        }
    }
}

