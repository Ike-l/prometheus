use std::time::Duration;

use crate::prelude::time_plugin::prelude::Tick;

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
    pub delay_progress: Delay,
    pub delay_target: Delay,
}

pub fn update_delayed_ui() {

}