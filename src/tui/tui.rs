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
    terminal,
};

use std::io;

use crate::player::audio_player::AudioPlayer;

pub struct Tui {
    player: AudioPlayer,
}

impl Tui {
    pub fn new(player: AudioPlayer) -> Self {
        Self { player }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal::enable_raw_mode()?;
        execute!(terminal.backend_mut(), cursor::Hide)?;

        terminal.clear()?;
        execute!(terminal.backend_mut(), terminal::Clear(terminal::ClearType::Purge))?;

        loop {
            terminal.draw(|f| {
                let area = f.area();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ])
                    .split(area);

                self.render_player(f, chunks[0]);
                self.render_controls(f, chunks[1]);
                self.render_shortcuts(f, chunks[2]);
            })?;

            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if self.handle_input(code)? {
                    break;
                }
            }
        }

        terminal.clear()?;
        execute!(terminal.backend_mut(), terminal::Clear(terminal::ClearType::Purge))?;

        terminal::disable_raw_mode()?;
        execute!(terminal.backend_mut(), cursor::Show)?;
        Ok(())
    }

    fn render_player(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let block = Block::default()
            .title("Audio Player")
            .borders(Borders::ALL);
        f.render_widget(block, area);
    }

    fn render_controls(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let play_pause_button = self.player.pause_or_play_button_text();
        let paragraph = Paragraph::new(play_pause_button)
            .block(Block::default().title("Control").borders(Borders::ALL));
        f.render_widget(paragraph, area);
    }

    fn render_shortcuts(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let controls = vec![
            "Space - Play/Pause",
            "R - Restart",
            "Esc or Q - Quit",
        ];
        let controls_text = Text::from(controls.join("\n"));
        let shortcuts = Paragraph::new(controls_text)
            .block(Block::default().title("Keyboard Shortcuts").borders(Borders::ALL));
        f.render_widget(shortcuts, area);
    }

    fn handle_input(&mut self, code: KeyCode) -> io::Result<bool> {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => Ok(true),
            KeyCode::Char(' ') => {
                self.player.toggle_playing();
                Ok(false)
            }
            KeyCode::Char('r') => {
                self.player.restart();
                self.player.toggle_playing();
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

