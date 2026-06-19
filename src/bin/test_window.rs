use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{ContentStyle, Print, PrintStyledContent, StyledContent};
use crossterm::queue;
use mancala::app::App;
use mancala::theme::Theme;
use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main() {
	let mut app = App::new();
	let mut stdout = stdout();


	let mut selected_element:Option<Rc<RefCell<dyn HandleKeypress>>> = None;
	let mut selectable_elements_list: Vec<Rc<RefCell<dyn HandleKeypress>>> = Vec::new();

	(selected_element, selectable_elements_list) = populate_window();

	let window = Rc::new(RefCell::new(
		Window{
			row: 0,
			column: 50,
			width: 20,
			height: 20,
			selectable_elements: vec![],
			is_highlighted: false,
			theme: Default::default(),
		}
	));
	selectable_elements_list.push(window);

	let target_fps = 60;
	let target_duration_micros = 1000000 / target_fps;
	let mut start_time;
	let mut sleep_time = 0u64;

	let mut event_consumer:Option<Rc<RefCell<dyn HandleKeypress>>> = None;

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
						// if key_event.code.is_left() {
						// 	selected_element = selectable_elements_list.first().cloned();
						// }
						// if key_event.code.is_right(){
						// 	selected_element = selectable_elements_list.last().cloned();
						// }

						event_consumer = match event_consumer{
							Some(event_consumer_ref) => {
								let response = {
									let mut consumer = event_consumer_ref.borrow_mut();
									consumer.handle_keypress(key_event)
								};
								match response {
								    KeyEventResponse::RemoveEventConsumer => {None}
									KeyEventResponse::ChangeEventConsumer(new_consumer) => {Some(new_consumer)}
									KeyEventResponse::KeepEventConsumer => {Some(event_consumer_ref)}
								}
							}
							None => {
								if matches!(key_event.code, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) {
									selected_element = move_to_new_selectable_element(
										selected_element,
										selectable_elements_list.as_slice(),
										key_event.code);
								}
								event_consumer
							}
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

fn populate_window() -> (Option<Rc<RefCell<dyn HandleKeypress>>>, Vec<Rc<RefCell<dyn HandleKeypress>>>) {
	let mut selected_element: Option<Rc<RefCell<dyn HandleKeypress>>>;
	let mut selectable_element_list: Vec<Rc<RefCell<dyn HandleKeypress>>> = Vec::new();

	let mut new_element: Rc<RefCell<dyn HandleKeypress>>;
	let window_elements = [
		(0, 10, 5, 1, "UP", false),
		(3, 5, 5, 1, "LEFT", false),
		(3, 10, 5, 1, "CENTER", false),
		(3, 16, 5, 1, "RIGTH", false),
		(5, 10, 5, 1, "DOWN", false),
	];

	for (row, column, width, height, text, is_highlighted) in window_elements.iter(){
		new_element = Rc::new(RefCell::new(
			InputTextField{
				row: *row,
				column: *column,
				width: *width,
				height: *height,
				text: text.to_string(),
				is_highlighted: *is_highlighted,
				theme: Default::default(),
			}
		));
		selectable_element_list.push(new_element);
	};
	selected_element = Some(selectable_element_list.first().unwrap().clone());
	selected_element.as_ref().unwrap().borrow_mut().switch_highlight(true);
	(selected_element, selectable_element_list)
}

fn move_to_new_selectable_element(
	old_selected_element: Option<Rc<RefCell<dyn HandleKeypress>>>,
	selectable_element_list: &[Rc<RefCell<dyn HandleKeypress>>],
	key_code: KeyCode,
) -> Option<Rc<RefCell<dyn HandleKeypress>>> {
	let old_rc = match old_selected_element {
		Some(el) => el,
		None => {
			if let Some(first) = selectable_element_list.first() {
				first.borrow_mut().switch_highlight(true);
				return Some(first.clone());
			}
			return None;
		}
	};

	let (row_c, col_c) = old_rc.borrow().get_elements_center();
	let mut nearest_element: Option<Rc<RefCell<dyn HandleKeypress>>>= None;
	let mut nearest_element_distance = u16::MAX;

	for element in selectable_element_list {
		if Rc::ptr_eq(element, &old_rc) {
			continue;
		}
		let (temp_row_c, temp_col_c) = element.borrow().get_elements_center();
		let is_in_direction = match key_code {
			KeyCode::Up => temp_row_c < row_c,    // Assuming 0 is top of screen
			KeyCode::Down => temp_row_c > row_c,  // Assuming max_val is bottom
			KeyCode::Left => temp_col_c < col_c,
			KeyCode::Right => temp_col_c > col_c,
			_ => false,
		};
		if is_in_direction {
			let distance = row_c.abs_diff(temp_row_c) + col_c.abs_diff(temp_col_c);

			if distance < nearest_element_distance {
				nearest_element = Some(element.clone());
				nearest_element_distance = distance;
			}
		}
	}

	if let Some(ref new_element) = nearest_element {
		old_rc.borrow_mut().switch_highlight(false);
		new_element.borrow_mut().switch_highlight(true);
		nearest_element
	} else {
		Some(old_rc.clone())
	}
}

trait HandleKeypress {
	fn handle_keypress(&mut self, key: KeyEvent) -> KeyEventResponse;
	fn display(&self, stdout: &mut Stdout);
	fn switch_highlight(&mut self, new_state: bool);
	fn get_elements_center(&self) -> (u16, u16);
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
	fn handle_keypress(&mut self, key: KeyEvent) -> KeyEventResponse {
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
		KeyEventResponse::KeepEventConsumer
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

	fn switch_highlight(&mut self, new_state: bool) {
		self.is_highlighted = new_state;
	}
	fn get_elements_center(&self) -> (u16, u16) {
		let row_c = self.row + (self.height / 2) as u16;
		let col_c = self.column + (self.width / 2) as u16;
		(row_c.clone(), col_c.clone())
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
	fn handle_keypress(&mut self, key: KeyEvent) -> KeyEventResponse{
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
		KeyEventResponse::KeepEventConsumer
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
	fn switch_highlight(&mut self, new_state: bool) {
		self.is_highlighted = new_state;
	}
	fn get_elements_center(&self) -> (u16, u16) {
		let row_c = self.row + (self.height / 2) as u16;
		let col_c = self.column + (self.width / 2) as u16;
		(row_c.clone(), col_c.clone())
	}
}

enum KeyEventResponse{
	KeepEventConsumer,
	RemoveEventConsumer,
	ChangeEventConsumer(Rc<RefCell<dyn HandleKeypress>>)
}

struct Window {
	row: u16,
	column: u16,
	width: usize,
	height: usize,
	selectable_elements: Vec<Rc<RefCell<dyn HandleKeypress>>>,
	is_highlighted: bool,
	theme: Theme,
}


impl HandleKeypress for Window {
	fn handle_keypress(&mut self, key: KeyEvent) -> KeyEventResponse {
		KeyEventResponse::KeepEventConsumer
	}

	fn display(&self, stdout: &mut Stdout) {

		let top_bottom_border = match self.is_highlighted{
			true => {StyledContent::new(self.theme.get_highlight_style(), "X".repeat(self.width +1))}
			false => {StyledContent::new(self.theme.get_content_style(), "X".repeat(self.width +1))}
		};
		let side_border = match self.is_highlighted{
			true => {StyledContent::new(self.theme.get_highlight_style(), "X")}
			false => {StyledContent::new(self.theme.get_content_style(), "X")}
		};

		for row in 0..self.height {
			if row == 0 || row == self.height - 1 {
				queue!(stdout,
					MoveTo(self.column, row as u16 + self.row),
					PrintStyledContent(top_bottom_border.clone())
				).unwrap()
			}
			else{
				queue!(stdout,
					MoveTo(self.column, row as u16 + self.row),
					PrintStyledContent(side_border),
					MoveTo(self.column + self.width as u16, row as u16 + self.row),
					PrintStyledContent(side_border)
				).unwrap()
			}
		}
	}

	fn switch_highlight(&mut self, new_state: bool) {
		self.is_highlighted = new_state;
	}

	fn get_elements_center(&self) -> (u16, u16) {
		let row_c = self.row + (self.height / 2) as u16;
		let col_c = self.column + (self.width / 2) as u16;
		(row_c.clone(), col_c.clone())
	}
}