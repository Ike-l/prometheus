use std::{any::{type_name, Any, TypeId}, cell::UnsafeCell, collections::{BTreeMap, HashMap}, ops::Bound::{Excluded, Included}};

use injection_types::{event::{queue::{EventQueue, EventQueueHandler}, reader::EventReader, writer::EventWriter}, world::command_queue::CommandQueue};
use ordered_float::OrderedFloat;
use system::{IntoSystem, System};

use crate::prelude::{Command, Event};

use super::app::world_registry::WorldRegistry;

pub mod injection_types;
pub mod system;
mod test;

pub type StoredSystem = Box<dyn System>;
pub type TypeMap = HashMap<TypeId, UnsafeCell<Box<dyn Any>>>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Access {
    Read,
    Write
}
pub type AccessMap = HashMap<TypeId, Access>;

#[derive(Debug, Default)]
pub struct Scheduler {
    systems: BTreeMap<OrderedFloat<f64>, Vec<StoredSystem>>,
    resources: TypeMap,
    accesses: AccessMap,
}

impl Scheduler {
    pub const INTERVAL: f64 = 1.;
    pub const START: f64 = 0.;
    pub const TICK: f64 = Scheduler::START + Scheduler::INTERVAL;
    pub const END: f64 = Scheduler::TICK + Scheduler::INTERVAL;
    pub const EXIT: f64 = Scheduler::END + Scheduler::INTERVAL;

    /// Runs the systems inserted between the bounds
    /// after each phase accesses is cleared
    /// at the end of a tick event queues and the command buffer is processed
    /// at the end of start the command buffer is processed
    pub fn run(&mut self, start: f64, end_exclusive: f64) {
        self.systems
            .range_mut(
                (
                    Included(OrderedFloat(start)),
                    Excluded(OrderedFloat(end_exclusive))       
                )
            )
            .for_each(
                |(_, systems)| {
                    systems
                        .iter_mut()
                        .for_each(
                            |system| system.run(&self.resources, &mut self.accesses)
                        );
                    self.accesses.clear();
                }
            );
        
        if start == Self::TICK { 
            self.process_event_queues();
            if let Err(e) = self.clear_command_queue() {
                log::warn!("{}", e)
            }
        }
    }

    /// panics if the phase isnt contained within the bounds START..EXIT
    pub fn insert_system<T, I, S>(&mut self, phase: f64, system: T) 
    where 
        T: IntoSystem<I, System = S>,
        S: System + 'static
    {
        assert!((Self::START..Self::EXIT).contains(&phase), "Phase expected between {} and {}. Found: {phase}", Self::START, Self::EXIT);

        self.systems
            .entry(OrderedFloat(phase))
            .or_default()
            .push(Box::new(system.into_system()));
    }

    pub fn register_event<E: Event>(&mut self) -> bool {
        let event_queue: Box<dyn EventQueueHandler> = Box::new(EventQueue::<E>::default());
        self.resources.insert(
            TypeId::of::<EventQueue<E>>(),
            UnsafeCell::new(
                Box::new(event_queue) as Box<dyn Any>
            )
        ).is_some()
    }

    pub fn retrieve_event_queue<E: Event>(&self) -> Option<&EventQueue<E>> {
        self.retrieve_resource::<Box<dyn EventQueueHandler>>()
            .and_then(|handler| handler.as_any().downcast_ref())
    }

    pub fn retrieve_event_reader<E: Event>(&self) -> Option<EventReader<E>> {
        self.retrieve_event_queue::<E>().map(EventReader::new)
    }

    pub fn retrieve_event_writer<E: Event>(&self) -> Option<EventWriter<E>> {
        self.resources.get(&TypeId::of::<EventQueue<E>>()).map(|cell| {
            let event_queue = unsafe { &mut *cell.get() };
            let handler = event_queue.downcast_mut::<Box<dyn EventQueueHandler>>().unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()));
            let queue = handler.as_any_mut().downcast_mut::<EventQueue<E>>().unwrap_or_else(|| panic!("Downcasting event: {}", type_name::<E>()));
            
            EventWriter::new(queue)
        })
    }
    
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> bool {
        let value = UnsafeCell::new(Box::new(resource));
        self.resources.insert(TypeId::of::<T>(), value).is_some()
    }

    pub fn clear_resource<T: 'static>(&mut self) -> bool {
        self.resources.remove(&TypeId::of::<T>()).is_some()
    }

    pub fn retrieve_resource<T: 'static>(&self) -> Option<&T> {
        unsafe {
            self.resources
                .get(&TypeId::of::<T>())
                .map(|cell| &*cell.get())
                .and_then(|boxed| boxed.downcast_ref::<T>())
        }
    }

    // Looks scary but accesses are tracked
    pub fn retrieve_resource_mut<T: 'static>(&self) -> Option<&mut T> {
        unsafe {
            self.resources
                .get(&TypeId::of::<T>())
                .map(|cell| &mut *cell.get())
                .and_then(|boxed| boxed.downcast_mut::<T>())
        }
    }

    pub fn retrieve_resource_all<T: 'static>(&self) -> Option<Vec<&T>> {
        let mut resources = vec![];

        for (_, resource) in self.resources.iter() {
            let resource = unsafe { &*resource.get() };
            if let Some(resource) = resource.downcast_ref::<T>() {
                resources.push(resource);
            }
        }

        if resources.len() == 0 {
            None
        } else {
            Some(resources)
        }
    }

    pub fn retrieve_resource_all_mut<T: 'static>(&mut self) -> Option<Vec<&mut T>> {
        let mut resources = vec![];

        for (_, resource) in self.resources.iter_mut() {
            let resource = unsafe { &mut *resource.get() };
            if let Some(resource) = resource.downcast_mut::<T>() {
                resources.push(resource);
            }
        }

        if resources.len() == 0 {
            None
        } else {
            Some(resources)
        }
    }

    fn clear_command_queue(&mut self) -> Result<(), &'static str> {
        let world = match self.retrieve_resource_mut::<hecs::World>() {
            Some(world) => world,
            None => return Err("Failed to retrieve world")
        };

        let world_registry = match self.retrieve_resource_mut::<WorldRegistry>() {
            Some(registry) => registry,
            None => return Err("Failed to retrieve world registry")
        };

        let cmd_buf = match self.retrieve_resource_mut::<CommandQueue>() {
            Some(cmd_buffer) => cmd_buffer,
            None => return Err("Failed to retrieve command buffer")
        };

        while let Some(command) = cmd_buf.pop() {
            match command {
                Command::Despawn(entity) => {
                    let _ = world_registry.remove_entity(&entity);
                    let _ = world.despawn(entity);
                },
                _ => {}
            }
        }
        
        Ok(())
    }

    fn process_event_queues(&mut self) {
        if let Some(mut resources) = self.retrieve_resource_all_mut::<Box<dyn EventQueueHandler>>() {
            resources.iter_mut().for_each(|resource| resource.tick());
        }
    }
}