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
    theme_list_window: ThemeListWindow,
    render_next: bool,
    theme_is_selected: bool,
}

impl Settings {
    pub fn new() -> Settings {
        let mut bindings:FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), Settings::exit_app);
        bindings.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), Settings::main_menu);
        bindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), Settings::move_up);
        bindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), Settings::move_down);

        Settings{
            bindings,
            theme_list_window: ThemeListWindow::new(),
            render_next: true,
            theme_is_selected: false,
        }
    }
    pub fn main_menu(self: Box<Settings>) -> Box<dyn State> {
        Box::new(MainMenu::new())
    }
    pub fn exit_app(self: Box<Settings>) -> Box<dyn State> {
        Box::new(ExitScreen::new())
    }
    pub fn print_theme_list(self, app: &App) {
        self.theme_list_window.print_window(app);
    }
    fn move_up(mut self: Box<Self>) -> Box<dyn State>{
        self.theme_list_window.move_theme_list_index_up(&mut self.render_next);
        self
    }
    fn move_down(mut self: Box<Self>) -> Box<dyn State>{
        self.theme_list_window.move_theme_list_index_down(&mut self.render_next);
        self
    }
    fn select_theme(mut self: Box<Self>) -> Box<dyn State> {
        self.theme_is_selected = true;
        self
    }
}

impl State for Settings {
    fn render(&mut self,app: &App) {
        self.render_next = !self.render_next;
        let mut stdout = stdout();
        let (_w, _h) = terminal::size().unwrap();
        let _current_theme = &app.theme;
        queue!(
            stdout,
            Clear(ClearType::All),
        ).unwrap();
        self.theme_list_window.print_window(&app);
        stdout.flush().unwrap();
    }

    fn handel_input(self: Box<Self>, event: Event, _app: &mut App) -> Box<dyn State> {
        match event {
            Event::Key(event) => {
                if event.is_press(){
                    if !self.theme_is_selected {
                        return match self.bindings.get(&event) {
                            Some(func) => { func(self) },
                            None => { self }
                        }
                    }
                    else {
                        //TODO Need to pass event to selected window to handle input
                        return self
                    }
                }
                self
            }
            _ => {self}
        }
    }

    fn render_next(&self, _app: &mut App) -> bool {
        self.render_next
    }
}


struct ThemeListWindow<>{
    window_width: u16,
    window_height: u16,
    starting_position_width: u16,
    starting_position_height: u16,
    theme_list: Vec<Theme>,
    theme_list_index: u16,
}

impl ThemeListWindow{
    fn new() -> ThemeListWindow{
        let mut theme_list: Vec<Theme> = Vec::with_capacity(10);
        theme_list.push(Theme::default());
        theme_list.push(Theme::ema());

        ThemeListWindow{
            window_width: 22,
            window_height: 15,
            starting_position_width: 0,
            starting_position_height: 0,
            theme_list: theme_list,
            theme_list_index: 0,
        }
    }

    fn move_theme_list_index_up(&mut self, render_next: &mut bool){
        if self.theme_list_index > 0{
            self.theme_list_index -= 1;
            *render_next = true;
        }
    }

    fn move_theme_list_index_down(&mut self, render_next: &mut bool){
        if self.theme_list_index + 1 < self.theme_list.len() as u16 {
            self.theme_list_index += 1;
            *render_next = true;
        }
    }

    fn print_window(&self, app: &App){
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