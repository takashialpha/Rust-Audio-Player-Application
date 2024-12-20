use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
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

        terminal.clear()?;
        execute!(terminal.backend_mut(), cursor::Hide)?;

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                    .split(f.area());

                let block = Block::default().title("Audio Player").borders(Borders::ALL);
                f.render_widget(block, chunks[0]);

                // Display Play/Pause button text
                let play_pause_button = self.player.pause_or_play_button_text();
                let paragraph = Paragraph::new(play_pause_button)
                    .block(Block::default().title("Control").borders(Borders::ALL));
                f.render_widget(paragraph, chunks[1]);
            })?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Char('p') => self.player.toggle_playing(),
                    KeyCode::Char('r') => self.player.restart(),
                    _ => {}
                }
            }
        }

        execute!(terminal.backend_mut(), cursor::Show)?;
        terminal.clear()?;
        Ok(())
    }
}
