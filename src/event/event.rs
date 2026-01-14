use crossterm::event::KeyEvent;

pub enum Event {
    Key(KeyEvent),
    Tick,
}