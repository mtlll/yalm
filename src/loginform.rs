use crate::textbox::Textbox;
use tui::{
    backend::Backend,
    layout::{
        Layout,
        Rect,
        Direction,
        Constraint
    },
    widgets::{
        Block,
        Paragraph,
        Borders,
        Widget,
        Text,
    },
    Frame,
    Terminal,
};
use crossterm::{
    event::{
        read,
        Event,
        KeyCode
    },
    ErrorKind,
};

pub struct LoginForm {
    message: Option<String>,
    error: Option<String>,
    inputs: Vec<Textbox>,
    inputs_idx: usize,
    block_height: u16
}


impl Default for LoginForm {
    fn default() -> LoginForm {
        LoginForm {
            message: None,
            error: None,
            inputs: vec![],
            inputs_idx: 0,
            block_height: 2
        }
    }
}

impl LoginForm {
    
    pub fn add_input(&mut self, title: String, mask_input: bool) {
        self.inputs.insert(self.inputs_idx, Textbox::default().title(title).mask_input(mask_input));
        self.inputs_idx += 1;
        self.block_height += 3;
    }
    
    pub fn draw_form<B>(&mut self, term: &mut Terminal<B>) 
    where
        B: Backend
    {
        term.draw(|mut f| {
            let (message_chunk, input_chunk) = self.get_form_chunks(f.size());
        
            self.draw_inputs(&mut f, input_chunk);
            self.draw_message(&mut f, message_chunk);
        }).unwrap();
    }
    
    pub fn message(&mut self, message: String) {
        self.message = Some(message); 
    }
    
    pub fn error(&mut self, error: String)
    {
        self.error = Some(error);
    }
    
    //reset the struct, but keep the error string as is for the next round
    fn reset(&mut self) {
        self.message = None;
        self.inputs.clear();
        self.inputs_idx = 0;
        self.block_height = 2;
    }
    
    fn get_form_chunks(&self, layout_chunk: Rect) -> (Rect, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(40)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Length(1),
                    Constraint::Length(self.block_height),
                    Constraint::Percentage(40),
                ]
                .as_ref()
            )
            .split(layout_chunk);
                (chunks[1], chunks[2])
    }

    fn draw_message<B>(&mut self, f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
    {
        if let Some(message) = self.error.as_ref().or(self.message.as_ref()) { 
            Paragraph::new([Text::raw(message)].iter()).render(f, layout_chunk);
        } 
    }
    fn draw_inputs<B>(&mut self, f: &mut Frame<B>, layout_chunk: Rect)
    where
        B: Backend
    {
        Block::default()
            .borders(Borders::ALL)
            .render(f, layout_chunk);
    
    
        let constraints = vec![Constraint::Length(3); self.inputs_idx];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(layout_chunk);
        
        for i in 0..self.inputs_idx {
            self.inputs[i].render(f, chunks[i]);
        }
    }
    
    pub fn get_next_input<B>(&mut self, term: &mut Terminal<B>) -> Result<String, ErrorKind>
    where
        B: Backend
    {
        let active_idx : usize;
        
        if self.inputs_idx > 0 {
            active_idx = self.inputs_idx - 1;
        } else {
            return Ok("".to_string());
        }
        
        term.show_cursor()?;
        
        loop {
            self.draw_form(term);
            let active_input : &mut Textbox = &mut self.inputs[active_idx];
            let (x, y) = active_input.get_cursor_xy();
            term.set_cursor(x, y)?;
            
            if let Ok(Event::Key(event)) = read() {
                match event.code {
                    KeyCode::Char(c) => {
                        active_input.add_char(c);
                    }
                    KeyCode::Backspace => {
                        active_input.remove_char();
                    }
                    KeyCode::Left => {
                        active_input.move_left();
                    }
                    KeyCode::Right => {
                        active_input.move_right();
                    }
                    KeyCode::Enter => {
                        term.hide_cursor()?;
                        return Ok(active_input.get_input());
                    }
                    KeyCode::Esc => {
                        panic!("foo")
                    }
                    _ => {}
                }
            }
        }
        
    }
}


