use std::io;
use std::io::Stdout;
use crossterm;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> anyhow::Result<Tui> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    
    Ok(terminal)
}

pub fn restore(mut terminal: Tui) -> anyhow::Result<()> {
    disable_raw_mode()?;
    
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    
    terminal.show_cursor()?;
    
    Ok(())
}

