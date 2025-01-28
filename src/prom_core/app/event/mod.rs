use event_driver::EventDriver;
use crate::prelude::Event;


use small_derive_deref::{Deref, DerefMut};
use winit::{event::{DeviceEvent, DeviceId, WindowEvent}, window::WindowId};

#[derive(Debug, Deref, DerefMut, EventDriver, Clone)]
pub struct WindowEventBus {
    #[DerefTarget]
    #[DerefMutTarget]
    event: WindowEvent,
    pub window_id: WindowId,
}

impl WindowEventBus {
    pub fn new(event: WindowEvent, window_id: WindowId) -> Self {
        Self {
            event,
            window_id
        }
    }
}


#[derive(Debug, Deref, DerefMut, EventDriver, Clone)]
pub struct DeviceEventBus {
    #[DerefTarget]
    #[DerefMutTarget]
    event: DeviceEvent,
    pub device_id: DeviceId
}

impl DeviceEventBus {
    pub fn new(event: DeviceEvent, device_id: DeviceId) -> Self {
        Self {
            event,
            device_id
        }
    }
}