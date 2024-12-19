mod player;
mod tui;
mod error;

use player::audio_player::AudioPlayer;
use tui::tui::Tui;
use std::path::PathBuf;

fn main() {
    let mut player = AudioPlayer::new();

    let path = PathBuf::from("path/to/your/audio.wav");
    if let Err(e) = player.play_file(path) {
        eprintln!("Error playing file: {}", e);
    }

    let mut tui = Tui::new(player);
    if let Err(e) = tui.run() {
        eprintln!("Error running the TUI: {}", e);
    }
}

