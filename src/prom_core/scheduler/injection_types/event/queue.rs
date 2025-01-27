use std::any::Any;

use super::Event;

/// Underlying data for EventWriter and EventReader
#[derive(Debug)]
pub struct EventQueue<E: Event> {
    pub events: Vec<(E, bool)>
}

impl<E: Event> Default for EventQueue<E> {
    fn default() -> Self {
        Self {
            events: vec![]
        }
    }
}

impl<E: Event> Event for EventQueue<E> {}

impl<E: Event> EventQueue<E> {
    pub fn push(&mut self, event: E) {
        self.events.push((event, false));
    }    

    /// Provides an iterator over all events without the used flag
    pub fn events(&self) -> impl Iterator<Item = &E> + '_ {
        self.events.iter().map(|(e, _)| e)
    }

    /// Ticking events allows it to be cleaned up
    pub fn tick_events(&mut self) {
        self.events.iter_mut().for_each(|(_, used)| *used = true);
    }

    /// Removes all events which have been ticked
    pub fn clean(&mut self) {
        self.events.retain(|&(_, used)| !used);
    }
}

pub trait EventQueueHandler {
    fn tick(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<E: Event> EventQueueHandler for EventQueue<E> {
    fn tick(&mut self) {
        self.tick_events();
        self.clean();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}