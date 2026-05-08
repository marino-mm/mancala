use std::cell::RefCell;
use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyEvent, KeyModifiers};
use crossterm::style::{Print, PrintStyledContent, StyledContent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::execute;
use mancala::app::App;
use mancala::theme::Theme;
use std::io::{stdout, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main(){
    let mut app = App::new();
    let mut stdout = stdout();

    // let mut input_field = InputTextField::new("".into(), 25, false, app.theme.clone());
    let mut input_field: Rc<RefCell<Box<dyn HandleKeypress>>> =
        Rc::new(
            RefCell::new(
                Box::new(
                    InputTextField::new("".into(), 25, false, app.theme.clone())
        )));
    let mut selected_element_stack:Vec<Rc<RefCell<Box<dyn HandleKeypress>>>> = Vec::with_capacity(10);
    selected_element_stack.push(input_field.clone());

    let target_fps = 60;
    let target_duration_micros = 1000000/target_fps;
    let mut start_time;
    let mut sleep_time = 0u64;

    while app.running{
        start_time = Instant::now();
        if poll(Duration::from_micros(sleep_time)).unwrap() {
            match read().unwrap() {
                Event::Key(key_event) => {
                    if key_event.is_press(){
                        if key_event.code.is_char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL){
                            app.running = false;
                            break;
                        }
                        selected_element_stack.last().unwrap().borrow_mut().handle_keypress(key_event);
                    // input_field.handle_keypress(key_event);
                    }
                }
                _ => {}
            }
        }
        execute!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::CurrentLine),
            PrintStyledContent(input_field.borrow().get_styled_content()),
        ).expect("Error putting content into queue");
        sleep_time = target_duration_micros - start_time.elapsed().as_micros() as u64;
    }
}

trait HandleKeypress{
    fn handle_keypress(&mut self, key: KeyEvent);
    fn get_styled_content(&self) -> StyledContent<String>;
}

struct InputTextField{
    text: String,
    max_width: usize,
    is_highlighted: bool,
    theme: Theme,
}

impl InputTextField{
    pub fn new(text: String, max_width:usize, is_highlighted: bool, theme: Theme) -> InputTextField{
        InputTextField{text, max_width, is_highlighted, theme}
    }
}

impl HandleKeypress for InputTextField{
    fn handle_keypress(&mut self, key: KeyEvent){
        match key.code.as_char() {
            None => {}
            Some(c) => {
                self.text.push(c);
                while self.max_width < self.text.chars().count(){
                    self.text.remove(0);
                }
            }
        }
        if key.code.is_backspace(){
            self.text.pop();
        }
        execute!(
            stdout(),
            MoveTo(0, 1),
            Clear(ClearType::CurrentLine),
            Print(format!("{:?}", key)),
        ).expect("Error putting content into queue");
    }

    fn get_styled_content(&self) -> StyledContent<String> {
        match self.is_highlighted{
            true => {
                StyledContent::new(self.theme.get_highlight_style(), self.text.clone())
            }
            false => {
                StyledContent::new(self.theme.get_content_style(), self.text.clone())
            }
        }
    }
}