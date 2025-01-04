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
    FalseStable,

    Pos,
    TruePos,

    Neg,
    TrueNeg
}

#[derive(Debug, Default)]
pub struct EdgedUIComponent {
    start_len: usize,
    pub edge: Edge
}

/*
Stable: start = now
FalseStable: start = now, start != 0

Pos: start < now
TruePos: start < now, start = 0

Neg: start > now
TrueNeg: start > now, now = 0
*/

impl EdgedUIComponent {
    pub fn state(&self, ui_event_buffer: &Vec<Event>) -> Edge {
        let now = ui_event_buffer.len();
        let start = self.start_len;
        // No change
        if start == now {
            // could have an event removed and added i.e changed so `False` 
            if start > 0 {
                Edge::FalseStable
            } else {
                Edge::Stable
            }
        // Positive change
        } else if start < now {
            // First change
            if start == 0 {
                Edge::TruePos
            } else {
                Edge::Pos
            }
        // Negative change
        // } else if start > now {
        } else {
            // Events clear (i.e last change)
            if now == 0 {
                Edge::TrueNeg
            } else {
                Edge::Neg
            }
        }
    }
}


pub fn store_lens(world: MutWorld) {
    for (_, (edged_ui, ui)) in &mut world.query::<(&mut EdgedUIComponent, &UIComponent)>() {
        edged_ui.start_len = ui.event_buffer.len()
    }
}