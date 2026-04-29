use crate::app::App;
use crate::screen::exit_screen::ExitScreen;
use crate::screen::main_menu::MainMenu;
use crate::screen::state::State;
use crate::theme::Theme;
use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Print, SetStyle};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};
use rustc_hash::FxHashMap;
use std::io::{stdout, Write};

pub struct Settings {
    bindings: FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>>,
    theme_list: Vec<Theme>,
    theme_list_index: usize,
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
            theme_list_index: 0,
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

    pub fn print_theme_list(&mut self, app: &App) {
        let mut window = ThemeListWindow::new(&self.theme_list);
        window.print_window(app);
    }
}

impl State for Settings {
    fn render(&mut self,app: &App) {
        let mut stdout = stdout();
        let (_w, _h) = terminal::size().unwrap();
        let _current_theme = &app.theme;
        queue!(
            stdout,
            Clear(ClearType::All),
        ).unwrap();
        self.print_theme_list(&app);
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


struct ThemeListWindow<'a>{
    window_width: u16,
    window_height: u16,
    starting_position_width: u16,
    starting_position_height: u16,
    theme_list: &'a Vec<Theme>,
    theme_list_index: u16,
}

impl ThemeListWindow<'_>{
    fn new(theme_list_window: &'_ Vec<Theme>) -> ThemeListWindow<'_>{
        ThemeListWindow{
            window_width: 22,
            window_height: 15,
            starting_position_width: 0,
            starting_position_height: 0,
            theme_list: theme_list_window,
            theme_list_index: 0,
        }
    }

    fn move_theme_list_index_up(&mut self){
        if self.theme_list_index > 0{
            self.theme_list_index -= 1;
        }
    }

    fn move_theme_list_index_down(&mut self){
        if self.theme_list_index + 1 < self.theme_list.len() as u16 {
            self.theme_list_index += 1;
        }
    }

    fn print_window(&mut self, app: &App){
        let content_width = (self.window_width -2) as usize;

        let border_top = format!("╔{}╗", "═".repeat(content_width));
        let border_bottom = format!("╚{}╝", "═".repeat(content_width));

        queue!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(self.starting_position_width, self.starting_position_height),
            Print(border_top),
            MoveToNextLine(1),
            MoveToColumn(self.starting_position_width)
        ).unwrap();

        for n in 0..self.window_height{
            let formated_line = match self.theme_list.get(n as usize) {
                Some(theme) => format!("║{: <width$}║", theme.to_string(), width = content_width),
                None => {format!("║{: <width$}║", " ".to_string(), width = content_width)}
            };
            if n != self.theme_list_index{
                queue!(
                    stdout(),
                    Print(formated_line),
                    MoveToNextLine(1),
                    MoveToColumn(self.starting_position_width)
                ).unwrap();
            } else {
                queue!(
                    stdout(),
                    SetStyle(app.theme.get_highlight_style()),
                    Print(formated_line),
                    SetStyle(app.theme.get_content_style()),
                    MoveToNextLine(1),
                    MoveToColumn(self.starting_position_width)
                ).unwrap();
            }
        }
        queue!(
                stdout(),
                Print(border_bottom),
                MoveToNextLine(1),
                MoveToColumn(self.starting_position_width)
            ).unwrap();
    }
}