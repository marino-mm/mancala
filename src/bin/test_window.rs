use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyEvent, KeyModifiers, poll, read};
use crossterm::execute;
use crossterm::style::{Print, PrintStyledContent, StyledContent};
use crossterm::terminal::{Clear, ClearType};
use mancala::app::App;
use mancala::theme::Theme;
use std::cell::RefCell;
use std::io::{Write, stdout};
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main() {
	let mut app = App::new();
	let mut stdout = stdout();

	// let mut input_field = InputTextField::new("".into(), 25, false, app.theme.clone());
	// let mut input_field: Rc<RefCell<Box<dyn HandleKeypress>>> =
	//     Rc::new(
	//         RefCell::new(
	//             Box::new(
	//                 InputTextField::new("".into(), 25, false, app.theme.clone())
	//     )));
	let mut input_field: Rc<RefCell<Box<dyn HandleKeypress>>> = Rc::new(RefCell::new(Box::new(InputNumberField::new(
		0,
		10,
		false,
		app.theme.clone(),
	))));
	let mut selected_element_stack: Vec<Rc<RefCell<Box<dyn HandleKeypress>>>> = Vec::with_capacity(10);
	selected_element_stack.push(input_field.clone());

	let target_fps = 60;
	let target_duration_micros = 1000000 / target_fps;
	let mut start_time;
	let mut sleep_time = 0u64;

	while app.running {
		start_time = Instant::now();
		if poll(Duration::from_micros(sleep_time)).unwrap() {
			match read().unwrap() {
				Event::Key(key_event) => {
					if key_event.is_press() {
						if key_event.code.is_char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
							app.running = false;
							break;
						}
						selected_element_stack
							.last()
							.unwrap()
							.borrow_mut()
							.handle_keypress(key_event);
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
		)
		.expect("Error putting content into queue");
		sleep_time = target_duration_micros - start_time.elapsed().as_micros() as u64;
	}
}

trait HandleKeypress {
	fn handle_keypress(&mut self, key: KeyEvent);
	fn get_styled_content(&self) -> StyledContent<String>;
}

struct InputTextField {
	text: String,
	max_width: usize,
	is_highlighted: bool,
	theme: Theme,
}

impl HandleKeypress for InputTextField {
	fn handle_keypress(&mut self, key: KeyEvent) {
		match key.code.as_char() {
			None => {}
			Some(c) => {
				self.text.push(c);
				while self.max_width < self.text.chars().count() {
					self.text.remove(0);
				}
			}
		}
		if key.code.is_backspace() | key.code.is_delete() {
			self.text.pop();
		}
		execute!(
			stdout(),
			MoveTo(0, 1),
			// Clear(ClearType::CurrentLine),
			Print(String::from(" ").repeat(self.max_width)),
			MoveTo(0, 1),
			Print(format!("{:?}", key)),
		)
		.expect("Error putting content into queue");
	}

	fn get_styled_content(&self) -> StyledContent<String> {
		match self.is_highlighted {
			true => StyledContent::new(self.theme.get_highlight_style(), self.text.clone()),
			false => StyledContent::new(self.theme.get_content_style(), self.text.clone()),
		}
	}
}

impl InputTextField {
	fn default() -> InputTextField {
		InputTextField {
			text: String::default(),
			max_width: 50,
			is_highlighted: false,
			theme: Default::default(),
		}
	}
	fn new(text: String, max_width: usize, is_highlighted: bool, theme: Theme) -> InputTextField {
		InputTextField {
			text,
			max_width,
			is_highlighted,
			theme,
		}
	}
	fn change_max_width(&mut self, max_width: usize) {
		self.max_width = max_width;
		while self.max_width < self.text.chars().count() {
			self.text.remove(0);
		}
	}
}

struct InputNumberField {
	content_number: i128,
	max_width: usize,
	is_highlighted: bool,
	theme: Theme,
}

impl InputNumberField {
	fn new(starting_number: i128, max_width: usize, is_highlighted: bool, theme: Theme) -> InputNumberField {
		InputNumberField {
			content_number: starting_number,
			max_width,
			is_highlighted,
			theme,
		}
	}
}

impl HandleKeypress for InputNumberField {
	fn handle_keypress(&mut self, key: KeyEvent) {
		match key.code.as_char() {
			None => {}
			Some(c) => {
				// if  c.is_digit(10){
				// self.content_number *= 10;
				// self.content_number += c as i128;
				// }
				match c.to_digit(10) {
					None => {}
					Some(n) => {
						self.content_number *= 10;
						if self.content_number >= 0 {
							self.content_number += n as i128;
						} else {
							self.content_number -= n as i128;
						}
					}
				}
				if c == '-' {
					if self.content_number > 0 {
						self.content_number *= -1;
					}
				}
				if c == '+' {
					if self.content_number < 0 {
						self.content_number *= -1;
					}
				}
				let mut number_text: String = String::with_capacity(128);
				loop {
					number_text.clear();
					number_text = self.content_number.to_string();
					if self.max_width > number_text.chars().count() {
						break;
					} else {
						if self.content_number > 0 {
							self.content_number %= 10_i128.pow(self.max_width as u32 - 1);
						}
						if self.content_number < 0 {
							self.content_number %= 10_i128.pow(self.max_width as u32 - 2);
						}
					}
				}
			}
		}
		if key.code.is_backspace() | key.code.is_delete() {
			self.content_number /= 10;
		}
		execute!(
			stdout(),
			MoveTo(0, 2),
			// Clear(ClearType::CurrentLine),
			Print(String::from(" ").repeat(self.max_width)),
			MoveTo(0, 2),
			Print(format!("{:?}", key)),
		)
		.expect("Error putting content into queue");
	}

	fn get_styled_content(&self) -> StyledContent<String> {
		match self.is_highlighted {
			true => StyledContent::new(self.theme.get_highlight_style(), self.content_number.to_string()),
			false => StyledContent::new(self.theme.get_content_style(), self.content_number.to_string()),
		}
	}
}
