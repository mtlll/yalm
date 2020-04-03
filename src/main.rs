use std::borrow::BorrowMut;
use std::io;
use std::{thread, time};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect, Margin};
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, Widget, Paragraph, Text};
use tui::Frame;
use tui::Terminal;
use crossterm::{
	terminal,
	cursor::MoveTo,
	ErrorKind,
	event::{read, Event, KeyCode}
};
use std::process::Command;
use std::env;
use pam::Authenticator;

use std::os::unix::process::CommandExt;

use users;
use users::os::unix::UserExt;

mod textbox;

use textbox::Textbox;

pub struct Inputs<'a> {
	inputs: [Textbox<'a>; 2],
	focused: Focused,
}

pub enum Focused {
	Username,
	Password
}

use crate::Focused::*;

impl<'a> Inputs<'a> {
	pub fn focused(&mut self) -> &mut Textbox<'a> {
		match self.focused {
			Username => {&mut self.inputs[0]}
			Password => {&mut self.inputs[1]}
		}
	}
	
	pub fn username(&mut self) -> &mut Textbox<'a> {
		&mut self.inputs[0]
	}
	
	pub fn password(&mut self) -> &mut Textbox<'a> {
		&mut self.inputs[1]
	}
}

fn main() -> Result<(), ErrorKind> {
	let stdout = io::stdout();
	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let mut inputs = Inputs {
		inputs: [
					Textbox::default().title("Username"),
					Textbox::default().title("Password").mask_input(true)
				],
		focused: Username
	};
	//let mut active_input : &Textbox = &inputs.username;
	
	terminal.clear()?;
	terminal.show_cursor()?;
	loop {
		terminal.draw(|mut f| {
			draw_main_layout(&mut f, &mut inputs);
		})?;
		
		let (x, y) = inputs.focused().get_cursor_xy();
		terminal.set_cursor(x, y)?;
		
		if let Event::Key(event) = read()? {
			//eprintln!("Received keyevent: {:?}", event);
			let input_mut = inputs.focused();
			match event.code {
				
				KeyCode::Esc => {
					break;
				}
				KeyCode::Char(c) => {
					input_mut.add_char(c);
				}
				KeyCode::Backspace => {
					input_mut.remove_char();
				}
				KeyCode::Left => {
					input_mut.move_left();
				}
				KeyCode::Right => {
					input_mut.move_right();
				}
				KeyCode::Enter => {
					match inputs.focused {
						Username => {
							inputs.focused = Password;
						}
						Password => {
							break;
						}
					}				
				}
				_ => {}
			}
			
		}
	}
    
    terminal.clear()?;
    let username = inputs.username().clone().get_input();
    let user = users::get_user_by_name(&username).expect("No such user.");
    let password = inputs.password().clone().get_input();
    let mut auth = Authenticator::with_password("login").expect("Failed to init PAM.");
    auth.get_handler().set_credentials(username, password);
    auth.authenticate().expect("Failed to auth.");
    auth.open_session().expect("Failed to open session");
    env::set_current_dir(user.home_dir())?;
    let mut child = Command::new("/usr/bin/zsh")
        .uid(user.uid())
        .gid(user.primary_group_id())
        .arg("-l")
        .arg("-c")
        .arg("startx")
        .spawn()?;
    child.wait()?;
    println!("\nSuccessfully created a session!");
	Ok(())
}

fn draw_main_layout<B>(f: &mut Frame<B>, inputs: &mut Inputs) 
where 
	B: Backend, 
{ 
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.horizontal_margin(40)
		.constraints(
			[
				Constraint::Percentage(40),
				Constraint::Length(8),
				Constraint::Percentage(40),
			]
			.as_ref(),
		)
		.split(f.size());
	Block::default()
		//.title("Foo")
		.borders(Borders::ALL)
		.render(f, chunks[1]);
	
	draw_inputs(f, chunks[1], inputs)
}

fn draw_inputs<B>(f: &mut Frame<B>, layout_chunk: Rect, inputs: &mut Inputs)
where
	B: Backend
{
	let chunks = Layout::default()
	
		.direction(Direction::Vertical)
		.margin(1)
		.constraints(
			[
				Constraint::Percentage(50),
				Constraint::Percentage(50),
			]
			.as_ref()
		)
		.split(layout_chunk);
		
	inputs.username().render(f, chunks[0]);
	inputs.password().render(f, chunks[1]);
	
}