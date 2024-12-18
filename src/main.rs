#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use audio_player::AudioPlayer;
use eframe::egui;

mod audio_player;

const SUPPORTED_FILE_EXTENSIONS: [&str; 2] = ["wav", "wave"];
const MIN_WINDOW_SIZE: [f32; 2] = [350.0, 150.0];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        //The window is currently created at its minimum size
        viewport: egui::ViewportBuilder::default().with_min_inner_size(MIN_WINDOW_SIZE).with_inner_size(MIN_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Audio Player Application",
        options,
        Box::new(|_cc| {
            Ok(Box::<Application>::default())
        }),
    )
}

struct Application {
    audio_player: AudioPlayer,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            audio_player: AudioPlayer::new(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(track_name) = self.audio_player.track_name() {
                ui.label(format!("Currently playing '{}'!", track_name));
            }
            
            if self.audio_player.is_playing() {
                ctx.request_repaint(); //Makes the frame repaint so that the progress bar is updated correctly
            }

            ui.add(egui::ProgressBar::new(self.audio_player.progress()));

            ui.vertical_centered_justified(|ui| {
                let button_text = self.audio_player.pause_or_play_button_text();
                if ui.button(button_text).clicked() {
                    self.audio_player.toggle_playing();
                }
            });
            
            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    if ui.button("Select file").clicked() {
                        let selected_path = rfd::FileDialog::new()
                            .add_filter("Audio", &SUPPORTED_FILE_EXTENSIONS)
                            .pick_file();

                        if let Some(path) = selected_path {
                            self.audio_player.play_file(path);
                        }
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if ui.button("Restart").clicked() {
                        self.audio_player.restart();
                    }
                });
            });
        });
    }
}
