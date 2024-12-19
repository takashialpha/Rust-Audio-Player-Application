use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    terminal::Terminal,
};
use crossterm::{
    ExecutableCommand, event::{self, KeyCode, KeyEvent}, terminal::{self, ClearType},
    cursor, execute, event::Event,
};
use std::io::{self, Write};

use crate::player::audio_player::AudioPlayer;
use crate::player::stream::StreamHandler;

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

        terminal.clear(ClearType::All)?;
        execute!(terminal.backend_mut(), cursor::Hide)?;

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default().title("Audio Player").borders(Borders::ALL);
                f.render_widget(block, size);

                let play_pause_button = self.player.pause_or_play_button_text();
                let paragraph = Paragraph::new(play_pause_button)
                    .block(Block::default().title("Control").borders(Borders::ALL));
                f.render_widget(paragraph, size);
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
        terminal.clear(ClearType::All)?;
        Ok(())
    }
}

