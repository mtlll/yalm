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

mod input;

use input::Inputs;

fn main() -> Result<(), ErrorKind> {
	let stdout = io::stdout();
	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let mut inputs = Inputs::default();
	//let mut active_input : &Textbox = &inputs.username;
	
    inputs.add_input("foo", false);
    inputs.add_input("bar", false);
	terminal.clear()?;
	loop {
        terminal.draw(|mut f| {
            inputs.draw_inputs(&mut f)
        })?;
		if let Event::Key(event) = read()? {
			//eprintln!("Received keyevent: {:?}", event);
			match event.code {
				
				KeyCode::Esc => {
					break;
				}
                KeyCode::Enter => {
                    inputs.add_input("foo", false);
                }
                _ => {}
			}
			
		}
	}
    
    terminal.clear()?;
	Ok(())
}

/*
fn oldmain() -> Result<(), ErrorKind> {
    
	let stdout = io::stdout();
	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let mut inputs = Inputs::default();
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
*/
