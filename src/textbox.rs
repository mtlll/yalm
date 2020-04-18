use tui::widgets::{
	Paragraph,
	Block,
	Widget,
	Borders,
	Text
};
use tui::layout::{
	Rect,
};
use tui::buffer::Buffer;

use tui::backend::Backend;

use tui::terminal::Frame;
#[derive(Clone)]
pub struct Textbox<> {
	input: Vec<char>,
	input_mask: Option<Vec<char>>,
	input_idx: usize,
	cursor_offset: u16,
	cursor_x: u16,
	cursor_y: u16,
	mask_input: bool,
	mask_char: char,
	mask_char_width: u16,
	is_focused: bool,
	title: Option<String>,
}

impl Default for Textbox {
	fn default() -> Textbox {
		Textbox {
			input: vec![],
			input_mask: None,
			input_idx: 0,
			cursor_offset: 0,
			cursor_x: 0,
			cursor_y: 0,
			mask_input: false,
			mask_char: '*',
			mask_char_width: 0,
			is_focused: false,
			title: None,
		}
	}
} 

impl Textbox {
	pub fn mask_input(mut self, flag: bool) -> Textbox  {
		self.mask_input = flag;
		self.input_mask = if flag {
			Some(vec![])
		} else {
			None
		};
		
		self
	}
	
	pub fn mask_char(mut self, mask_char: char) -> Textbox {
		self.mask_char = mask_char;
		self
	}
	
	pub fn title(mut self, title: String) -> Textbox {
		self.title = Some(title);
		self
	}
	
	pub fn add_char(&mut self, c: char) {
		self.input.insert(self.input_idx, c);
		if self.mask_input {
			self.input_mask.as_mut().unwrap().insert(self.input_idx, self.mask_char);
			self.cursor_offset += 1; //TODO: calculate this properly accounting for unicode characters
		} else {
			self.cursor_offset += 1;
		}
		self.input_idx += 1;
		
	}
	
	pub fn remove_char(&mut self) {
		if self.input_idx > 0 {
			self.input_idx -= 1;
			self.input.remove(self.input_idx);
			if self.mask_input {
				self.input_mask.as_mut().unwrap().remove(self.input_idx);
				self.cursor_offset -= 1;
			} else {
				self.cursor_offset -= 1;
			}
		}
	}
	
	pub fn move_left(&mut self) {
		if self.input_idx > 0 {
			self.input_idx -= 1;
			self.cursor_offset -= 1;
		}
	}
	
	pub fn move_right(&mut self) {
		if self.input_idx < self.input.len() {
			self.input_idx += 1;
			self.cursor_offset += 1;
		}
	}
	
	pub fn get_cursor_xy (&self) -> (u16, u16) {
		(self.cursor_x, self.cursor_y)
	}
	
	pub fn get_input(&self) -> String {
		self.input.iter().collect()
	}
}

impl Textbox
{
	pub fn render<B>(&mut self, f: &mut Frame<B>, area: Rect) 
	where
		B: Backend
	{
		let field_width = (area.width - 3) as usize;
		let display_vec : &Vec<char>= if let Some(vec) = &self.input_mask {
			&vec
		} else {
			&self.input
		};
		
		let display_string: String = if self.input_idx >= field_width {
			let (_, end_string) = display_vec.split_at(self.input_idx - field_width);
			end_string.iter().collect()
		} else {
			display_vec.iter().collect()
		};
		
		let block = if let Some(title) = &self.title {
			Block::default().borders(Borders::ALL).title(&title)
		} else {
			Block::default().borders(Borders::ALL)
		};
		
		self.cursor_x = area.x + 1 + if self.cursor_offset > field_width as u16{
			field_width as u16
		} else {
			self.cursor_offset
		};
		
		self.cursor_y = area.y + 1;
		
		Paragraph::new([Text::raw(display_string)].iter())
			.block(block)
			.render(f, area);
	}
}