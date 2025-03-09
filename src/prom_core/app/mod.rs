use std::any::type_name;

use world_registry::WorldRegistry;
use event::{DeviceEventBus, WindowEventBus};
use hecs::World;
use winit::{application::ApplicationHandler, event::{DeviceEvent, DeviceId, StartCause, WindowEvent}, event_loop::ActiveEventLoop, window::WindowId};

use crate::{prelude::Event, prom_core::scheduler::injection_types::world::command_queue::CommandQueue};

use super::{scheduler::{system::{IntoSystem, System}, Scheduler}, unity::plugin::Plugin};

pub mod event;
pub mod app_builder;
pub mod world_registry;

#[derive(Debug, Default)]
pub struct App {
    scheduler: Scheduler,
    title: &'static str,
    /// Use offset to mitigate independent access crashes by setting it before adding systems/plugins and setting it back afterwards
    pub phase_offset: f64,
}

impl App {
    pub fn title(&self) -> &'static str {
        self.title
    }    

    pub fn insert_plugin(&mut self, plugin: Box<dyn Plugin>) { 
        log::info!("Inserting plugin: {}", plugin.id());
        plugin.build(self);
    }

    pub fn insert_plugins<T>(&mut self, plugins: T) where T: Iterator<Item = Box<dyn Plugin>> {
        plugins.for_each(|plugin| self.insert_plugin(plugin));
    }

    pub fn insert_system<T, I, S>(&mut self, phase: f64, system: T) 
    where 
        T: IntoSystem<I, System = S>,
        S: System + 'static
    {
        log::info!("Inserting system into phase: {:?}", phase + self.phase_offset);
        self.scheduler.insert_system(phase + self.phase_offset, system);
    }

    pub fn register_event<T: Event>(&mut self) {
        log::info!("Registering event: {:?}", type_name::<T>());
        if self.scheduler.register_event::<T>() {
            log::warn!("Replaced event: {:?}", type_name::<T>());
        }
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        log::info!("Inserting resoure: {:?}", type_name::<T>());
        if self.scheduler.insert_resource(resource) {
            log::warn!("Replaced resource: {:?}", type_name::<T>());
        }
    }

    pub fn insert_default_resource<T: 'static + Default>(&mut self) {
        self.insert_resource(T::default());
    }

    pub fn clear_resource<T: 'static>(&mut self) {
        log::info!("Clearing resource: {:?}", type_name::<T>());
        if !self.scheduler.clear_resource::<T>() {
            log::warn!("Failed clearing resource: {:?}. Resource did not exist", type_name::<T>());
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Resumed App");
        self.phase_offset = 0.0;
        //self.insert_system(Scheduler::END-f64::MIN_POSITIVE, loop_start);
        
        let event_loop_ptr: *const ActiveEventLoop = event_loop;
        let app_ptr: *mut App = self;
        unsafe {
            let event_loop_ref: &ActiveEventLoop = &*event_loop_ptr;
            let app_ref: &mut App = &mut *app_ptr;
            // Safe since access is tracked and is sequential
            self.insert_resource(event_loop_ref);
            self.insert_resource(app_ref);
        }

        self.insert_default_resource::<World>();
        self.insert_default_resource::<WorldRegistry>();
        self.insert_default_resource::<CommandQueue>();

        self.register_event::<WindowEventBus>();
        self.register_event::<DeviceEventBus>();

        log::info!("Scheduler running START");

        self.scheduler.run(Scheduler::START, Scheduler::TICK);

        self.clear_resource::<&ActiveEventLoop>();
        self.clear_resource::<&mut App>();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        log::info!("Received window event: {event:?}");

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Scheduler running END");
                self.scheduler.run(Scheduler::END, Scheduler::EXIT);
                event_loop.exit();
            },
            _ => {
                match self.scheduler.retrieve_event_writer::<WindowEventBus>() {
                    Some(mut bus) => {
                        bus.send(WindowEventBus::new(event, window_id));
                    },
                    None => {
                        log::warn!("Failed to retrieve event writer of WindowEventBus")
                    }
                }
            }
        }
    }
    
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        log::info!("Received device event: {event:?}");
        
        match self.scheduler.retrieve_event_writer::<DeviceEventBus>() {
            Some(mut bus) => {
                bus.send(DeviceEventBus::new(event, device_id));
            },
            None => {
                log::warn!("Failed to retrieve event writer of DeviceEventBus")
            }
        }
    }
    
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if cause == StartCause::Poll {
            log::info!("Scheduler running TICK");
            self.scheduler.run(Scheduler::TICK, Scheduler::END);
        }
    }
}