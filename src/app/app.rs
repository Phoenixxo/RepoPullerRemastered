use crate::app::mode::AppMode;

pub struct App {
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Running,
        }
    }
    
    pub fn should_quit(&self) -> bool {
        self.mode == AppMode::Quit
    }
}