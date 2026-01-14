use crossterm::event::KeyCode;
use crate::app::app::App;
use crate::app::mode::AppMode;
use crate::event::Event;

pub fn handle_event(app: &mut App, event: Event) {
    match event {
        Event::Key(key) => {
            if key.code == KeyCode::Char('q') {
                app.mode = AppMode::Quit;
            }
        }
        
        Event::Tick => {}
    }
}

