use crate::app::App;
use crossterm::cursor::{MoveRight, MoveTo, MoveToNextLine};
use crossterm::event::{Event, KeyModifiers};
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::queue;
use std::io::{stdout, Write};
use crate::screen::exit_screen::ExitScreen;
use crate::screen::state::State;

pub struct MainMenu {
    selected_index: usize,
    render_next: bool,
    menu_items: Vec<&'static str>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            selected_index: 0,
            render_next: true,
            menu_items: vec!["Start Game", "Settings", "Exit"],
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
            Clear(ClearType::All),
            MoveTo(0, 0),
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
                    SetForegroundColor(app.theme.background),
                    SetBackgroundColor(app.theme.foreground),
                    Print(item),
                    ResetColor,
                    MoveToNextLine(1)
                ).unwrap();
            }
            else {
                queue!(stdout,
                    Clear(ClearType::CurrentLine),
                    MoveRight(((w/2) as usize - item_width_half) as u16),
                    Print(item),
                    MoveToNextLine(1)
                ).unwrap();
            }
        }
        stdout.flush().unwrap();
    }

    fn handel_input(mut self: Box<Self>, event: Event<>, _app: &mut App) -> Box<dyn State> {
        match event {
            Event::Key(event) => {
                if event.is_press(){
                    if event.code.is_char('c') && event.modifiers == KeyModifiers::CONTROL{
                        return Box::new(ExitScreen::new())
                    } else if event.code.is_down(){
                        if self.selected_index < self.menu_items.len() -1{
                            self.selected_index += 1;
                            self.render_next = true;
                        }
                        return self
                    } else if event.code.is_up(){
                        if self.selected_index > 0{
                            self.selected_index -= 1;
                            self.render_next = true;
                        }
                        return self
                    }
                    return self
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