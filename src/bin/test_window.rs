use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyEvent, KeyModifiers};
use crossterm::{execute, queue};
use crossterm::style::{ContentStyle, Print, PrintStyledContent, StyledContent};
use crossterm::terminal::{Clear, ClearType};
use mancala::app::App;
use mancala::theme::Theme;
use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main() {
	let mut app = App::new();
	let mut stdout = stdout();

	let mut string_input_field: Rc<RefCell<Box<dyn HandleKeypress>>> =
	    Rc::new(
	        RefCell::new(
	            Box::new(
	                InputTextField{
						row: 0,
		                column: 0,
		                width: 5,
		                height: 2,
		                text: "".to_string(),
		                is_highlighted: true,
		                theme: app.theme.clone(),
	                }
	    )));

	let mut number_input_field: Rc<RefCell<Box<dyn HandleKeypress>>> =
	Rc::new(RefCell::new(Box::new(InputNumberField {
		row: 0,
		column: 10,
		width: 10,
		height: 1,
		content_number: 0,
		is_highlighted: false,
		theme: app.theme.clone(),
	})));

	let mut selected_element:Option<Rc<RefCell<Box<dyn HandleKeypress>>>> = None;
	let mut selectable_elements_list: Vec<Rc<RefCell<Box<dyn HandleKeypress>>>> = Vec::new();

	selectable_elements_list.push(string_input_field.clone());
	selectable_elements_list.push(number_input_field.clone());

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
						if key_event.code.is_left() {
							selected_element = selectable_elements_list.first().cloned();
						}
						if key_event.code.is_right(){
							selected_element = selectable_elements_list.last().cloned();
						}
						match selected_element {
							Some(ref e) => {
								e.borrow_mut().handle_keypress(key_event);
							}
							None => {}
						}
					}
				}
				_ => {}
			}
		}
		for elem in selectable_elements_list.iter(){
			elem.borrow().display(&mut stdout)
		}
		stdout.flush().unwrap();
		sleep_time = target_duration_micros - start_time.elapsed().as_micros() as u64;
	}
}

trait HandleKeypress {
	fn handle_keypress(&mut self, key: KeyEvent);
	fn display(&self, stdout: &mut Stdout);

}

struct InputTextField {
	row: u16,
	column: u16,
	width: usize,
	height: usize,
	text: String,
	is_highlighted: bool,
	theme: Theme,
}

impl HandleKeypress for InputTextField {
	fn handle_keypress(&mut self, key: KeyEvent) {
		match key.code.as_char() {
			None => {}
			Some(c) => {
				self.text.push(c);
				while (self.width * self.height) < self.text.chars().count() {
					self.text.remove(0);
				}
			}
		}
		if key.code.is_backspace() | key.code.is_delete() {
			self.text.pop();
		}
	}

	fn display(&self, stdout: &mut Stdout) {
		let split_text: Vec<String> = self.text
			.chars()
			.collect::<Vec<char>>()
			.chunks(self.width)
			.map(|c| c.iter().collect::<String>())
			.collect();

		let mut styled_text;
		for row in 0..self.height {
			let text_line = match split_text.get(row){
				Some(l) => l,
				None => "",
			};
			styled_text = StyledContent::new(self.get_current_style(), text_line);
			queue!(
				stdout,
				MoveTo(self.column, self.row + row as u16),
				Print(" ".repeat(self.width)),
				MoveTo(self.column, self.row + row as u16),
				PrintStyledContent(styled_text),
			).unwrap()
		}
	}
}

impl InputTextField {
	fn default() -> InputTextField {
		InputTextField {
			row: 0,
			column: 0,
			text: String::default(),
			width: 50,
			is_highlighted: false,
			theme: Default::default(),
			height: 0,
		}
	}
	fn new(row: u16, column: u16, text: String, width: usize, height: usize, is_highlighted: bool, theme: Theme) -> InputTextField {
		InputTextField {
			row,
			column,
			text,
			width,
			is_highlighted,
			theme,
			height,
		}
	}

	fn get_current_style(&self) -> ContentStyle {
		match self.is_highlighted {
			true => self.theme.get_highlight_style(),
			false => self.theme.get_content_style()
		}
	}
}

struct InputNumberField {
	row: u16,
	column: u16,
	width: usize,
	height: usize,
	content_number: i128,
	is_highlighted: bool,
	theme: Theme,
}

impl InputNumberField {
	fn new(row: u16, column: u16, width: usize, height: usize, starting_number:i128, is_highlighted: bool, theme: Theme) -> InputNumberField {
		InputNumberField {
			row,
			column,
			width,
			height,
			content_number: starting_number,
			is_highlighted,
			theme,
		}
	}

	fn get_current_style(&self) -> ContentStyle {
		match self.is_highlighted {
			true => self.theme.get_highlight_style(),
			false => self.theme.get_content_style()
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
					if (self.width * self.height) > number_text.chars().count() {
						break;
					} else {
						if self.content_number > 0 {
							self.content_number %= 10_i128.pow((self.width * self.height) as u32 - 1);
						}
						if self.content_number < 0 {
							self.content_number %= 10_i128.pow((self.width * self.height) as u32 - 2);
						}
					}
				}
			}
		}
		if key.code.is_backspace() | key.code.is_delete() {
			self.content_number /= 10;
		}
	}
	fn display(&self, stdout: &mut Stdout) {

		let text = self.content_number.to_string();

		let split_text: Vec<String> = text
			.chars()
			.collect::<Vec<char>>()
			.chunks(self.width)
			.map(|c| c.iter().collect::<String>())
			.collect();

		let mut styled_text;
		for row in 0..self.height {
			let text_line = match split_text.get(row){
				Some(l) => l,
				None => "",
			};
			styled_text = StyledContent::new(self.get_current_style(), text_line);
			queue!(
				stdout,
				MoveTo(self.column, self.row + row as u16),
				Print(" ".repeat(self.width)),
				MoveTo(self.column, self.row + row as u16),
				PrintStyledContent(styled_text),
			).unwrap()
		}
	}
}
