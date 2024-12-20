use crossterm::event::{self, KeyEvent};

pub fn capture_input() -> Option<KeyEvent> {
    if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
        if let Ok(event) = event::read() {
            match event {
                event::Event::Key(key_event) => Some(key_event),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
