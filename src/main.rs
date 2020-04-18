use std::{env, io, os::unix::process::CommandExt, process::Command};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crossterm::{terminal, ErrorKind};

use pam::{Authenticator, Converse, PamError, PamResult};

use users;

use users::os::unix::UserExt;

mod textbox;

mod loginform;

mod dynamicconv;
use dynamicconv::DynamicConv;

static ERROR_STRINGS: [&str; 9] = [
    "Success. Wait, this is an error string?",
    "dlopen() failure when dynamically loading a service module",
    "Symbol not found",
    "Error in service module",
    "System error",
    "Memory buffer error",
    "Permission denied",
    "Authentication failure",
    "Unknown error",
];

fn main() -> Result<(), ErrorKind> {
    loop {
        main_loop()?;
    }
}
fn main_loop() -> Result<(), ErrorKind> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let user: users::User;
    let mut error: Option<String> = None;

    terminal::enable_raw_mode()?;
    term.clear()?;

    loop {
        match auth(&mut term, error) {
            Ok(username) => {
                user = users::get_user_by_name(&username).unwrap();
                break;
            }

            Err(PamError(errcode)) => {
                error =
                    Some(ERROR_STRINGS[clamp(errcode as usize, 0, ERROR_STRINGS.len())].to_string())
            }
        }
    }

    terminal::disable_raw_mode()?;
    term.clear()?;
    term.show_cursor()?;

    env::set_current_dir(user.home_dir()).expect("blahhh");
    let mut child = Command::new(user.shell())
        .uid(user.uid())
        .gid(user.primary_group_id())
        .arg("-l")
        .arg("-c")
        .arg("startx")
        .spawn()
        .expect("Oh dear god wat");
    child.wait().expect("The horror");

    Ok(())
}

fn clamp(input: usize, min: usize, max: usize) -> usize {
    if input > max {
        max
    } else if input < min {
        min
    } else {
        input
    }
}

/* Attempt to authenticate. Return a username if successful */
fn auth<B>(term: &mut Terminal<B>, error: Option<String>) -> PamResult<String>
where
    B: Backend,
{
    let mut conv = DynamicConv::new(term);
    if let Some(err) = error {
        conv.error_string(err);
    }

    let mut auth = Authenticator::with_handler("login", conv)?;
    auth.close_on_drop = false;

    auth.authenticate()?;
    auth.open_session()?;

    Ok(auth.handler().username().to_string())
}
