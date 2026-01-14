mod app;
mod lib;
mod tui;
mod event;

use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::{Block, Borders};
use crate::app::App;
use crate::app::handler::handle_event;
use crate::event::next_event;
use crate::tui::terminal;

fn main() -> anyhow::Result<()> {
    let mut terminal = terminal::init()?;
    let mut app = App::new();

    while !app.should_quit() {
        terminal.draw(|frame| {
            let block = Block::default()
                .title("Repo Puller Remastered")
                .borders(Borders::ALL);

            frame.render_widget(block, frame.size());
        })?;

        if let Some(event) = next_event(Duration::from_millis(250))? {
            handle_event(&mut app, event);
        }
    }

    terminal::restore(terminal)?;

    Ok(())
}
