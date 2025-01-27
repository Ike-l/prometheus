use std::any::{type_name, TypeId};

use crate::prom_core::scheduler::{injection_types::resource::referenced::Res, system::SystemParam, Access, AccessMap, TypeMap};

use super::{queue::{EventQueue, EventQueueHandler}, Event};

/// Provides read access to the events in the queue
#[derive(Debug)]
pub struct EventReader<'a, E: Event> {
    events: Res<'a, EventQueue<E>>
}

impl<'a, E: Event> EventReader<'a, E> {
    pub fn new(queue: &'a EventQueue<E>) -> Self {
        Self {
            events: Res { value: queue }
        }
    }
}

impl<'res, E: Event> SystemParam for EventReader<'res, E> {
    type Item<'new> = EventReader<'new, E>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<EventQueue<E>>()).or_insert(Access::Read), Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", type_name::<E>()
        );
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let unsafe_cell = resources
            .get(&TypeId::of::<EventQueue<E>>())
            .unwrap_or_else(|| panic!("Expected event in queue: {}", type_name::<E>()));
        
        let value_box = &*unsafe_cell.get();
        let value = value_box
            .downcast_ref::<Box<dyn EventQueueHandler>>()
            .unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()))
            .as_any()
            .downcast_ref::<EventQueue<E>>()
            .unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()));

        EventReader::new(value)
    }
}

impl<'a, E: Event> EventReader<'a, E> {
    pub fn read(&self) -> impl Iterator<Item = &E> + '_ {
        self.events.events()        
    }
}
