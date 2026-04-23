use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use std::io::{stdout, Stdout};

pub struct App{
    pub stdout: Stdout,
    pub running: bool
}

impl App {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout,
            EnterAlternateScreen,
            Clear(ClearType::Purge),
            SetTitle("This is a title"),
            MoveTo(0, 0),
            Hide
        ).unwrap();
        enable_raw_mode().expect("TODO: panic message");
        App {
            stdout,
            running: true,
        }
    }
}
impl Drop for App {
    fn drop (&mut self) {
        disable_raw_mode().expect("TODO: panic message");
        execute!(self.stdout,
            Show,
            LeaveAlternateScreen
        ).expect("TODOO");
    }
}