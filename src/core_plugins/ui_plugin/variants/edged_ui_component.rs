// when buffer is 0, edge: None
// when buffer is added to, edge: Some(Edge::Up)
// before buffer is cleared, edge: None
// when buffer is cleared, edge: Some(Edge::Down)

// to code a rising or falling edge tracker:
// check at beginning of tick, store whether it has something in it.
// check whether this has changed, if it has (nothing to something) then it is UP, if not (something to nothing) it is DOWN

use crate::prelude::{ui_plugin::prelude::{Event, UIComponent}, MutWorld};

#[derive(Debug, Default, PartialEq)]
pub enum Edge {
    #[default]
    Stable,
    Pos,
    Neg,
}

#[derive(Debug, Default)]
pub struct EdgedUIComponent {
    start_len: usize,
    pub edge: Edge
}

impl EdgedUIComponent {
    pub fn state(&self, ui_event_buffer: &Vec<Event>) -> Edge {
        if self.start_len == ui_event_buffer.len() {
            Edge::Stable
        } else if ui_event_buffer.len() > self.start_len {
            Edge::Pos
        } else {
            Edge::Neg
        }
    }
}


pub fn store_lens(world: MutWorld) {
    for (_, (edged_ui, ui)) in &mut world.query::<(&mut EdgedUIComponent, &UIComponent)>() {
        edged_ui.start_len = ui.event_buffer.len()
    }
}