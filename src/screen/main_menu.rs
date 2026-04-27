use crate::app::App;
use crate::screen::exit_screen::ExitScreen;
use crate::screen::state::State;
use crossterm::cursor::{MoveRight, MoveTo, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::queue;
use crossterm::style::{Print, SetStyle};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use rustc_hash::FxHashMap;
use crate::screen::game_screen::GameScreen;
use crate::screen::settings_screen::Settings;

pub struct MainMenu {
    selected_index: usize,
    render_next: bool,
    menu_items: Vec<&'static str>,
    bindings: FxHashMap<KeyEvent, fn(Box<MainMenu>) -> Box<dyn State>>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut bindings:FxHashMap<KeyEvent, fn(Box<MainMenu>) -> Box<dyn State>> = FxHashMap::default();
        bindings.insert(KeyEvent::new(KeyCode::Up, KeyModifiers::empty()), MainMenu::handel_up);
        bindings.insert(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()), MainMenu::handel_down);
        bindings.insert(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL), MainMenu::handel_exit);
        bindings.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()), MainMenu::handel_enter);

        MainMenu {
            selected_index: 0,
            render_next: true,
            menu_items: vec!["Start Game", "Settings", "Exit"],
            bindings,
        }
    }
    pub fn handel_up(mut self: Box<Self>) -> Box<dyn State> {
        if self.selected_index > 0{
            self.selected_index -= 1;
            self.render_next = true;
        }
        self
    }

    pub fn handel_down(mut self: Box<Self>) -> Box<dyn State> {
        if self.selected_index < self.menu_items.len() -1{
            self.selected_index += 1;
            self.render_next = true;
        }
        self
    }

    pub fn handel_exit(self: Box<Self>) -> Box<dyn State> {
        Box::new(ExitScreen::new())
    }
    pub fn handel_enter(self: Box<Self>) -> Box<dyn State> {
        match self.selected_index {
            0 => { Box::new(GameScreen::new()) },
            1 => { Box::new(Settings::new()) },
            2 => { Box::new(ExitScreen::new()) },
            _ => self
        }
    }
}

impl State for MainMenu {
    fn render(&mut self, app: &App) {
        self.render_next = !self.render_next;

        let mut stdout = stdout();
        let (w, _h) = crossterm::terminal::size().unwrap();

        let text = "███╗   ███╗ █████╗ ███╗   ██╗ ██████╗ █████╗ ██╗      █████╗
████╗ ████║██╔══██╗████╗  ██║██╔════╝██╔══██╗██║     ██╔══██╗
██╔████╔██║███████║██╔██╗ ██║██║     ███████║██║     ███████║
██║╚██╔╝██║██╔══██║██║╚██╗██║██║     ██╔══██║██║     ██╔══██║
██║ ╚═╝ ██║██║  ██║██║ ╚████║╚██████╗██║  ██║███████╗██║  ██║
╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝";
        queue!(stdout,
            SetStyle(app.theme.get_content_style()),
            Clear(ClearType::All),
            MoveTo(0, 2),
        ).unwrap();


        for line in text.lines() {
            let item_width_half = line.chars().count()/2;
            queue!(stdout,
                Clear(ClearType::CurrentLine),
                MoveRight((w/2) - item_width_half as u16),
                Print(line),
                MoveToNextLine(1),
            ).unwrap();
        }

        queue!(stdout,
            MoveToNextLine(2)
        ).unwrap();

        for (index, &item) in self.menu_items.iter().enumerate() {
            let item_width = item.chars().count();
            let item_width_half = item_width/2;

            if index == self.selected_index {
                queue!(stdout,
                    Clear(ClearType::CurrentLine),
                    MoveRight(((w/2) as usize - item_width_half) as u16),
                    SetStyle(app.theme.get_highlight_style()),
                    Print(item),
                    SetStyle(app.theme.get_content_style()),
                    MoveToNextLine(1)
                ).unwrap();
            }
            else {
                queue!(stdout,
                    Clear(ClearType::CurrentLine),
                    MoveRight(((w/2) as usize - item_width_half) as u16),
                    SetStyle(app.theme.get_content_style()),
                    Print(item),
                    MoveToNextLine(1)
                ).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
    fn handel_input(self: Box<Self>, event: Event<>, _app: &mut App) -> Box<dyn State> {
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
        self.render_next
    }
}