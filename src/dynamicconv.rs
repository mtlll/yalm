use std::ffi::{CStr, CString};

use tui::{backend::Backend, Terminal};

use pam::Converse;

use crate::loginform::LoginForm;

pub struct DynamicConv<'a, B>
where
    B: Backend,
{
    username: Option<String>,
    form: LoginForm,
    term: &'a mut Terminal<B>,
}

impl<'a, B> DynamicConv<'a, B>
where
    B: Backend,
{
    pub fn new(term: &'a mut Terminal<B>) -> DynamicConv<'a, B> {
        DynamicConv {
            username: None,
            form: LoginForm::default(),
            term,
        }
    }

    pub fn error_string(&mut self, error: String) {
        self.form.error(error);
    }
}

impl<B> Converse for DynamicConv<'_, B>
where
    B: Backend,
{
    fn prompt_echo(&mut self, msg: &CStr) -> Result<CString, ()> {
        let title = msg.to_string_lossy().to_string();
        self.form.add_input(title.clone(), false);

        if let Ok(input) = self.form.get_next_input(self.term) {
            if title == "login:" {
                self.username = Some(input.clone());
            }

            if let Ok(ret) = CString::new(input) {
                Ok(ret)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn prompt_blind(&mut self, msg: &CStr) -> Result<CString, ()> {
        self.form
            .add_input(msg.to_string_lossy().to_owned().to_string(), true);
        if let Ok(input) = self.form.get_next_input(self.term) {
            if let Ok(ret) = CString::new(input) {
                Ok(ret)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    fn info(&mut self, msg: &CStr) {
        self.form
            .message(msg.to_string_lossy().to_owned().to_string());
        self.form.draw_form(self.term);
    }

    fn error(&mut self, msg: &CStr) {
        self.form
            .error(msg.to_string_lossy().to_owned().to_string());
        self.form.draw_form(self.term);
    }

    fn username(&self) -> &str {
        if let Some(ret) = &self.username {
            &ret
        } else {
            ""
        }
    }
}
