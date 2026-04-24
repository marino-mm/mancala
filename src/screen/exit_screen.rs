use std::io::{stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::event::Event;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crate::app::App;
use crate::screen::state::State;

pub struct ExitScreen {
    render_next: bool,
}

impl ExitScreen {
    pub(crate) fn new() -> ExitScreen {
        ExitScreen { render_next: true }
    }
}

impl State for ExitScreen {
    fn render(&mut self, _app: &App) {
        let (w, h) = crossterm::terminal::size().unwrap();
        let mut stdout = stdout();
        let text = "Press any key to exit";
        queue!(stdout,
            Clear(ClearType::All),
            MoveTo((w/2) - (text.chars().count() /2) as u16, h/2),
            Print(text),
        ).unwrap();
        stdout.flush().unwrap();
    }

    fn handel_input(self: Box<Self>, _event: Event, app: &mut App) -> Box<dyn State> {
        app.running = false;
        self
    }

    fn render_next(&self, app: &mut App) -> bool {
        if self.render_next {
            app.running = false;
            return true;
        }
        false
    }
}