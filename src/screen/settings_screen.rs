use crate::app::App;
use crate::screen::exit_screen::ExitScreen;
use crate::screen::main_menu::MainMenu;
use crate::screen::state::State;
use crate::theme::{color_to_rgb, Theme};
use crossterm::cursor::{MoveTo, MoveToColumn, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, SetBackgroundColor, SetStyle};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};
use rustc_hash::FxHashMap;
use std::io::{stdout, Write};

pub struct Settings {
    bindings: FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>>,
    theme_list_window: ThemeListWindow,
    theme_detail_window: ThemeDetailWindow,
    render_next: bool,
    theme_is_selected: bool,
}

impl Settings {
    pub fn new() -> Settings {
        let mut bindings:FxHashMap<KeyEvent, fn(Box<Settings>) -> Box<dyn State>> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), Settings::exit_app);
        bindings.insert(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), Settings::main_menu);
        // bindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), Settings::move_up);
        // bindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), Settings::move_down);

        Settings{
            bindings,
            theme_list_window: ThemeListWindow::new(),
            theme_detail_window: ThemeDetailWindow::new(),
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
    fn _select_theme(mut self: Box<Self>) -> Box<dyn State> {
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
        self.theme_detail_window.print_window(&app, &self);
        stdout.flush().unwrap();
    }

    fn handel_input(mut self: Box<Self>, event: Event, _app: &mut App) -> Box<dyn State> {
        match event {
            Event::Key(event) => {
                // if event.is_press(){
                //     return if !self.theme_is_selected {
                //         match self.bindings.get(&event) {
                //             Some(func) => { func(self) },
                //             None => {
                //                 //TODO Need to pass event to selected window to handle input
                //                 self.theme_list_window.handle_input(event, &mut self.render_next);
                //                 return self
                //             }
                //         }
                //     } else {
                //         self
                //     }
                // }
                match self.bindings.get(&event) {
                    //This handles bindings for switching state
                    Some(func) => { func(self) },
                    None => {
                        //This passes events to windows
                        if !self.theme_is_selected {
                            self.theme_list_window.handle_input(event, &mut self.render_next);
                        } else {
                            self.theme_detail_window.handle_input(event, &mut self.render_next);
                        }
                        return self
                    },
                }
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
    bindings: FxHashMap<KeyEvent, fn(&mut ThemeListWindow, &mut bool)>
}

impl ThemeListWindow{
    fn new() -> ThemeListWindow{
        let mut theme_list: Vec<Theme> = Vec::with_capacity(10);
        theme_list.push(Theme::default());
        theme_list.push(Theme::ema());

        let mut bindings:FxHashMap<KeyEvent, fn(&mut ThemeListWindow, &mut bool)> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE), ThemeListWindow::move_theme_list_index_up);
        bindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE), ThemeListWindow::move_theme_list_index_down);

        ThemeListWindow{
            window_width: 22,
            window_height: 15,
            starting_position_width: 0,
            starting_position_height: 0,
            theme_list,
            theme_list_index: 0,
            bindings,
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
    fn get_selected_theme(&self) -> &Theme {
        self.theme_list.get(self.theme_list_index as usize).unwrap()
    }

    fn handle_input(&mut self, event: KeyEvent, render_next: &mut bool) {
        match self.bindings.get(&event) {
            Some(func) => { func(self, render_next) },
            None => {}
        }
    }
}


struct ThemeDetailWindow{
    window_width: u16,
    window_height: u16,
    starting_position_width: u16,
    starting_position_height: u16,
}

impl ThemeDetailWindow {
    pub(crate) fn handle_input(&self, _event: KeyEvent, _render_next: &mut bool) {
        // todo!()
    }
}

impl ThemeDetailWindow{
    fn new() -> ThemeDetailWindow{
        ThemeDetailWindow{
            window_width: 40,
            window_height: 15,
            starting_position_width: 24,
            starting_position_height: 0,
        }
    }

    fn print_window(&self, app: &App, state: &Settings) {
        // let current_theme = &app.theme;
        let current_theme = state.theme_list_window.get_selected_theme();
        let content_width = (self.window_width -2) as usize;

        let border_top = format!("╔{}╗", "═".repeat(content_width));
        let border_bottom = format!("╚{}╝", "═".repeat(content_width));

        queue!(
            stdout(),
            // Clear(ClearType::All),
            MoveTo(self.starting_position_width, self.starting_position_height),
            Print(border_top),
            MoveToNextLine(1),
            MoveToColumn(self.starting_position_width)
        ).unwrap();

        let (mut r,mut g,mut b): (u8, u8, u8);
        let mut display_line: String;
        let mut lines : Vec<(String, &Color)> = Vec::new();

        (r, g, b) = color_to_rgb(&current_theme.foreground);
        display_line = format!(" foreground:   r:{: >3}, g:{: >3}, b:{: >3}", r, g, b);
        lines.push((display_line, &current_theme.foreground));

        (r, g, b) = color_to_rgb(&current_theme.background);
        display_line = format!(" background:   r:{: >3}, g:{: >3}, b:{: >3}", r, g, b);
        lines.push((display_line, &current_theme.background));

        (r, g, b) = color_to_rgb(&current_theme.highlighted_foreground);
        display_line = format!(" foreground_h: r:{: >3}, g:{: >3}, b:{: >3}", r, g, b);
        lines.push((display_line, &current_theme.highlighted_foreground));

        (r, g, b) = color_to_rgb(&current_theme.highlighted_background);
        display_line = format!(" background_h: r:{: >3}, g:{: >3}, b:{: >3}", r, g, b);
        lines.push((display_line, &current_theme.highlighted_background));

        for (text, color) in lines.iter(){
            queue!(
                stdout(),
                Print(text),
                Print("  "),
                SetBackgroundColor(**color),
                Print("  "),
                SetStyle(app.theme.get_content_style()),
                MoveToNextLine(1),
                MoveToColumn(self.starting_position_width)
            ).unwrap();
        }

        queue!(
            stdout(),
            SetStyle(current_theme.get_content_style()),
            Print("Some text"),
            MoveToNextLine(1),
            MoveToColumn(self.starting_position_width)
        ).unwrap();

        queue!(
            stdout(),
            SetStyle(current_theme.get_highlight_style()),
            Print("Some highlighted text"),
            SetStyle(current_theme.get_content_style()),
            // MoveToNextLine(1),
            MoveToNextLine(self.window_height - 5),
            MoveToColumn(self.starting_position_width)
        ).unwrap();


        queue!(
            stdout(),
            SetStyle(app.theme.get_content_style()),
            Print(border_bottom),
            MoveToNextLine(1),
            MoveToColumn(self.starting_position_width)
        ).unwrap();
    }
}