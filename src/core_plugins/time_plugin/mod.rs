mod tick;
mod time;
mod accumulators;

pub mod prelude {
    pub use super::{
        UPDATE_TICK, UPDATE_TIME,
        tick::Tick, 
        time::{
            Time, fps_counter
        },
        accumulators::{
            Accumulator, Accumulators
        }
    };
}

use crate::prelude::*;

pub struct TimePlugin;

pub const UPDATE_TIME: f64 = 1.001;
pub const UPDATE_TICK: f64 = 1.001;

impl PluginTrait for TimePlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(UPDATE_TIME, time::update_time);
        app.add_system(UPDATE_TICK, tick::update_tick_count);
        app.add_resource(time::Time::default());
        app.add_resource(tick::Tick::default());
        app.add_resource(accumulators::Accumulators::default());

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_TimePlugin")
    }
}






