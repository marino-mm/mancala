use crate::app::App;
use crate::screen::exit_screen::ExitScreen;
use crate::screen::main_menu::MainMenu;
use crate::screen::state::State;
use crate::theme::Theme;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};
use rustc_hash::FxHashMap;
use std::io::{stdout, Write};

pub struct Settings {
    bindings: FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>>,
    theme_list: Vec<Theme>,
    selected_item_index: usize,
}

impl Settings {
    pub fn new() -> Settings {
        let mut bindings:FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), Settings::exit_app);
        bindings.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), Settings::main_menu);

        let theme_list: Vec<Theme> = Settings::load_stored_themes();

        Settings{
            bindings,
            theme_list,
            selected_item_index: 0,
        }
    }
    pub fn main_menu(self: Box<Settings>) -> Box<dyn State> {
        Box::new(MainMenu::new())
    }
    pub fn exit_app(self: Box<Settings>) -> Box<dyn State> {
        Box::new(ExitScreen::new())
    }

    pub fn load_stored_themes() -> Vec<Theme> {
        let mut theme_list: Vec<Theme> = Vec::with_capacity(10);
        theme_list.push(Theme::default());
        theme_list.push(Theme::ema());
        theme_list
    }
}

impl State for Settings {
    fn render(&mut self, app: &App) {
        let mut stdout = stdout();
        let (_w, _h) = terminal::size().unwrap();
        let _current_theme = &app.theme;
        queue!(
            stdout,
            Clear(ClearType::All),
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