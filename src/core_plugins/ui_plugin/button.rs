use std::time::Duration;

use super::{time_plugin::prelude::Tick, DeviceEventBus, WindowEventBus};

#[derive(Debug)]
pub enum Event {
    Window(WindowEventBus),
    Device(DeviceEventBus)
}

#[derive(Debug, Default)]
pub struct ToggleUIComponent {
    pub event: Option<Event>,
}

#[derive(Debug)]
pub enum Delay {
    Time(Duration),
    Tick(Tick),
}

impl Default for Delay {
    fn default() -> Self {
        Self::Tick(Tick(1))
    }
}


#[derive(Debug, Default)]
pub struct DelayedUIComponent {
    pub event: Option<Event>,
    delay_progress: Delay,
    delay_target: Delay,
}


