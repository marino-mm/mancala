use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;

pub struct App{
    pub running: bool,
    pub theme: Theme
}

pub struct Theme{
    pub foreground: Color,
    pub background: Color,
}

impl App {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout,
            EnterAlternateScreen,
            Clear(ClearType::Purge),
            MoveTo(0, 0),
            Hide
        ).unwrap();
        enable_raw_mode().expect("TODO: panic message");
        App {
            running: true,
            // theme: Theme{foreground: Color::White, background: Color::Black},
            theme: Theme{foreground: Color::Red, background: Color::Yellow},
        }
    }
}
impl Drop for App {
    fn drop (&mut self) {
        let mut stdout = stdout();
        disable_raw_mode().expect("TODO: panic message");
        execute!(stdout,
            Show,
            LeaveAlternateScreen
        ).expect("TODOO");
    }
}