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

