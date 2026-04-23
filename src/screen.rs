use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyModifiers};
use crossterm::execute;
use crossterm::style::Print;
use crate::app::{App};

pub trait State {
    fn render(&self, app: &App);
    fn handel_input(self: Box<Self>, event: Event, app: &mut App) -> Box<dyn State>;
}

pub struct MainMenu {}

impl State for MainMenu {
    fn render(&self, app: &App) {
        let (w, h) = crossterm::terminal::size().unwrap();

        let w_half = w / 2;
        let h_half = h / 2;

        let text = "Ovo je test za sredinu ekrana";
        let text_len = text.len();
        let text_len_half = (text_len / 2) as u16;

        execute!(
            &app.stdout,
            MoveTo(w_half - text_len_half, h_half),
            Print(text),
        ).expect("REASON")

    }

    fn handel_input(self: Box<Self>, event: Event<>, _app: &mut App) -> Box<dyn State> {
        match event {
            Event::Key(event) => {
                if event.is_press(){
                    if event.code.is_char('c') && event.modifiers == KeyModifiers::CONTROL{
                        return Box::new(ExitScreen{})
                    }
                    return self
                }
                self
            }
            _ => {self}
        }
    }
}

pub struct GameScreen {}

pub struct Settings {}

pub struct ExitScreen {}

impl State for ExitScreen {
    fn render(&self, _app: &App) {
        // todo!()
    }

    fn handel_input(self: Box<Self>, _event: Event, app: &mut App) -> Box<dyn State> {
        app.running = false;
        self
    }
}
