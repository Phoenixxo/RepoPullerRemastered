use std::time::Duration;
use crossterm::event::{self, Event as CrosstermEvent};
use crate::event::event::Event;

pub fn next_event(timeout: Duration) -> anyhow::Result<Option<Event>> {
    if event::poll(timeout)? {
        match event::read()? {
            CrosstermEvent::Key(key) => Ok(Some(Event::Key(key))),
            _ => Ok(None),
        }
    } else {
        Ok(Some(Event::Tick))
    }
}