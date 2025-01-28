use std::any::type_name;

use event::{DeviceEventBus, WindowEventBus};
use hecs::{CommandBuffer, World};
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

use crate::{prelude::Event, prom_core::core_plugin::loop_start};

use super::{scheduler::{system::{IntoSystem, System}, Scheduler}, unity::plugin::Plugin};

pub mod event;
pub mod app_builder;

#[derive(Debug, Default)]
pub struct App {
    scheduler: Scheduler,
    /// Use offset to mitigate independent access crashes by setting it before adding systems/plugins and setting it back afterwards
    pub phase_offset: f64,
}

impl App {
    pub fn insert_plugin(&mut self, plugin: Box<dyn Plugin>) { 
        log::info!("Inserting plugin: {}", plugin.id());
        plugin.build(self);
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
        self.scheduler.register_event::<T>();
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        log::info!("Inserting resoure: {:?}", type_name::<T>());
        self.scheduler.insert_resource(resource);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("Resumed App");
        self.phase_offset = 0.0;
        self.insert_system(Scheduler::END-f64::MIN_POSITIVE, loop_start);

        let event_loop_ptr: *const ActiveEventLoop = event_loop;
        let app_ptr: *mut App = self;
        unsafe {
            let event_loop_ref: &ActiveEventLoop = &*event_loop_ptr;
            let app_ref: &mut App = &mut *app_ptr;
            // Safe since access is tracked and is sequential
            self.scheduler.insert_resource(event_loop_ref);
            self.scheduler.insert_resource(app_ref);
        }
        log::info!("Inserted resources: &ActiveEventLoop, &mut App");

        self.scheduler.insert_resource(World::new());
        self.scheduler.insert_resource(CommandBuffer::new());

        self.scheduler.register_event::<WindowEventBus>();
        self.scheduler.register_event::<DeviceEventBus>();

        log::info!("Starting scheduler");

        self.scheduler.run(Scheduler::START, Scheduler::TICK);

        self.scheduler.clear_resource::<&ActiveEventLoop>();
        self.scheduler.clear_resource::<&mut App>();
        log::info!("Cleared resources: &ActiveEventLoop, &mut App");

        log::info!("Starting first tick");
        self.scheduler.run(Scheduler::TICK, Scheduler::END);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        log::info!("Received window event: {event:?}");

        let redraw = event == WindowEvent::RedrawRequested;

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Exiting App");
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

        if redraw {
            log::info!("Running Scheduler tick");
            self.scheduler.run(Scheduler::TICK, Scheduler::END);
        }
    }

    fn device_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
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
}