mod player;
mod tui;

use player::audio_player::AudioPlayer;
use tui::tui::Tui;
use std::path::PathBuf;

fn main() {
    let player = AudioPlayer::new();

    // Here you can provide the audio file path to the player
    let path = PathBuf::from("path/to/your/audio.wav");
    player.play_file(path);

    // Start the TUI interface
    let mut tui = Tui::new(player);
    if let Err(e) = tui.run() {
        eprintln!("Error running the TUI: {}", e);
    }
}

