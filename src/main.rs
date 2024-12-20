mod error;
mod player;
mod tui;

use player::audio_player::AudioPlayer;
use std::path::PathBuf;
use tui::tui::Tui;

fn main() {
    let mut player = AudioPlayer::new();

    let path = PathBuf::from("/home/takashi/Downloads/sound.wav");
    if let Err(e) = player.play_file(path) {
        eprintln!("Error playing file: {}", e);
    }

    let mut tui = Tui::new(player);
    if let Err(e) = tui.run() {
        eprintln!("Error running the TUI: {}", e);
    }
}
