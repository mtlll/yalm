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
        Borders,
        Widget,
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

pub struct Inputs<'a> {
    inputs: Vec<Textbox<'a>>,
    inputs_idx: usize,
    block_height: u16
}


impl<'a> Default for Inputs<'a> {
    fn default() -> Inputs<'a> {
        Inputs {
            inputs: vec![],
            inputs_idx: 0,
            block_height: 2
        }
    }
}

impl<'a> Inputs<'a> {
    
    pub fn add_input(&mut self, title: &'a str, mask_input: bool) {
        self.inputs.insert(self.inputs_idx, Textbox::default().title(title).mask_input(mask_input));
        self.inputs_idx += 1;
        self.block_height += 3;
    }
    
    pub fn draw_inputs<B>(&mut self, f: &mut Frame<B>) 
    where
        B: Backend
    {
        let layout_chunk = get_input_area(f.size(), self.block_height);
        
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
            term.draw(|mut f| {
                self.draw_inputs(&mut f);
            })?;
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
                        term.hide_cursor();
                        return Ok(active_input.get_input());
                    }
                    _ => {}
                }
            }
        }
        
    }
}

fn get_input_area(layout_chunk: Rect, height: u16) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(40)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Length(height),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(layout_chunk);
    
        chunks[1]
}

