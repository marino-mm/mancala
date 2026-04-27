use crate::app::App;
use crate::screen::exit_screen::ExitScreen;
use crate::screen::state::State;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};
use rustc_hash::FxHashMap;
use std::io::{stdout, Write};
use crate::screen::main_menu::MainMenu;

pub struct GameScreen {
    bindings: FxHashMap<KeyEvent, fn(Box<GameScreen>) -> Box<dyn State>>,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let mut bindings:FxHashMap<KeyEvent, fn(Box<GameScreen>) -> Box<dyn State>> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), GameScreen::exit_app);
        bindings.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), GameScreen::main_menu);

        GameScreen{
            bindings
        }
    }
    pub fn exit_app(self: Box<GameScreen>) -> Box<dyn State> {
        Box::new(ExitScreen::new())
    }
    pub fn main_menu(self: Box<GameScreen>) -> Box<dyn State> {Box::new(MainMenu::new())
    }
}

impl State for GameScreen {
    fn render(&mut self, _app: &App) {
        let mut stdout = stdout();
        let (w, h) = terminal::size().unwrap();

        let text = "This is the Game Screen";

        queue!(
            stdout,
            Clear(ClearType::All),
            MoveTo((w/2) - (text.chars().count()/2) as u16, h/2),
            Print(text)
        ).unwrap();

        stdout.flush().unwrap();
    }

    fn handel_input(self: Box<Self>, event: Event, _app: &mut App) -> Box<dyn State> {
        match event {
            Event::Key(event) => {
                if event.is_press(){
                    return match self.bindings.get(&event) {
                        Some(func) => { func(self) },
                        None => { self }
                    }
                }
                self
            }
            _ => {self}
        }
    }

    fn render_next(&self, _app: &mut App) -> bool {
        true
    }
}