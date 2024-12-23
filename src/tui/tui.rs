use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    text::{Text},
    Terminal,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

use std::io;

use crate::player::audio_player::AudioPlayer;

pub struct Tui {
    player: AudioPlayer,
}

impl Tui {
    pub fn new(player: AudioPlayer) -> Tui {
        Tui { player }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Enter raw mode
        terminal::enable_raw_mode()?;

        // Clear the terminal screen before starting
        terminal.clear()?;
        execute!(terminal.backend_mut(), cursor::Hide)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(5),   // Minimum space for player control
                        Constraint::Length(3), // Fixed space for Play/Pause button
                        Constraint::Length(3), // Fixed space for Keyboard Shortcuts
                    ])
                    .split(f.area()); // Ensure it takes up the whole screen

                let block = Block::default().title("Audio Player").borders(Borders::ALL);
                f.render_widget(block, chunks[0]);

                // Display Play/Pause button text
                let play_pause_button = self.player.pause_or_play_button_text();
                let paragraph = Paragraph::new(play_pause_button)
                    .block(Block::default().title("Control").borders(Borders::ALL));
                f.render_widget(paragraph, chunks[1]);

                // Define and display the keyboard shortcuts
                let controls = vec![
                    "Space - Play/Pause",
                    "R - Restart",
                    "Esc or Q - Quit",
                ];

                // Combine all control lines into a single Text using a String
                let controls_text = controls.join("\n"); // Join the lines with newline characters
                let controls_text = Text::from(controls_text); // Convert to Text

                // Create a Paragraph with the shortcuts
                let shortcuts = Paragraph::new(controls_text)
                    .block(Block::default().title("Keyboard Shortcuts").borders(Borders::ALL));

                // Render the shortcuts widget with a fixed height constraint to prevent scrolling
                f.render_widget(shortcuts, chunks[2]);
            })?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => self.player.toggle_playing(),
                    KeyCode::Char('r') => {
                        self.player.restart();
                        self.player.toggle_playing(); // Ensure playing state is set correctly after restart
                    }
                    _ => {}
                }
            }
        }

        // Disable raw mode
        terminal::disable_raw_mode()?;

        // Clear the terminal screen after the program ends
        terminal.clear()?;
        execute!(terminal.backend_mut(), cursor::Show)?;
        Ok(())
    }
}

