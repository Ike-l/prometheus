use std::any::{type_name, TypeId};

use crate::prom_core::scheduler::{injection_types::resource::mut_referenced::ResMut, system::SystemParam, Access, AccessMap, TypeMap};

use super::{queue::{EventQueue, EventQueueHandler}, Event};

/// Provides sending access to events in the queue
#[derive(Debug)]
pub struct EventWriter<'a, E: Event> {
    events: ResMut<'a, EventQueue<E>>,
}

impl<'a, E: Event> EventWriter<'a, E> {
    pub fn new(queue: &'a mut EventQueue<E>) -> Self {
        Self {
            events: ResMut { value: queue }
        }
    }
}

impl<'res, E: Event> SystemParam for EventWriter<'res, E> {
    type Item<'new> = EventWriter<'new, E>;

    fn accesses(access: &mut AccessMap) {
        match access.insert(TypeId::of::<EventQueue<E>>(), Access::Write) {
            Some(Access::Read) => panic!(
                "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", type_name::<E>()
            ),
            _  => (),
        }
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let unsafe_cell = resources
            .get(&TypeId::of::<EventQueue<E>>())
            .unwrap_or_else(|| panic!("Expected event in queue: {}", type_name::<E>()));
        
        let value_box = &mut *unsafe_cell.get();
        let value = value_box
            .downcast_mut::<Box<dyn EventQueueHandler>>()
            .unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()))
            .as_any_mut()
            .downcast_mut::<EventQueue<E>>()
            .unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()));

        EventWriter::new(value)
    }
}

impl<'a, E: Event> EventWriter<'a, E> {
    pub fn send(&mut self, event: E) {
        self.events.push(event);
    }
}
