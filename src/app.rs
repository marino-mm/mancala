use crate::theme::Theme;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{
	Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};
use std::io::{Write, stdout};

pub struct App {
	pub running: bool,
	pub theme: Theme,
}
impl App {
	pub fn new() -> Self {
		let mut stdout = stdout();
		queue!(
			stdout,
			EnterAlternateScreen,
			Clear(ClearType::All),
			SetTitle("Mancala"),
			MoveTo(0, 0),
			Hide
		)
		.unwrap();
		enable_raw_mode().unwrap();
		stdout.flush().unwrap();
		App {
			running: true,
			// theme: Theme::ema()
			theme: Theme::default(),
		}
	}
}
impl Drop for App {
	fn drop(&mut self) {
		let mut stdout = stdout();
		disable_raw_mode().expect("TODO: panic message");
		execute!(stdout, Show, LeaveAlternateScreen).expect("TODOO");
	}
}
