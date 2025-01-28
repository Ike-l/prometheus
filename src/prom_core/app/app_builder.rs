use small_derive_deref::{Deref, DerefMut};
use winit::{error::EventLoopError, event_loop::{ControlFlow, EventLoop}};

use super::App;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct AppBuilder {
    app: App,
}

impl AppBuilder {
    pub fn run(mut self) -> Result<(), EventLoopError> {
        env_logger::init();
        log::info!("Building app");

        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self.app)
    }
}