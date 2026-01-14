mod app;
mod lib;
mod tui;

use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::widgets::{Block, Borders};
use crate::tui::terminal;

fn main() -> anyhow::Result<()> {
    let mut terminal = terminal::init()?;

    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .title("Classroom Puller TUI")
                .borders(Borders::ALL);

            frame.render_widget(block, frame.size())
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal::restore(terminal);

    Ok(())
}
