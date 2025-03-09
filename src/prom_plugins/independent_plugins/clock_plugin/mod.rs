use std::collections::HashMap;

use clock::Clock;
use small_derive_deref::{Deref, DerefMut};

use crate::prelude::{App, Plugin, ResMut, Scheduler};

pub mod clock;

#[derive(Debug, Default, Clone, Copy, Deref, DerefMut)]
pub struct InternalClock {
    clock: Clock
}

#[derive(Debug, Clone)]
pub struct ClockPlugin {
    map: HashMap<String, f64>
}

impl Default for ClockPlugin {
    fn default() -> Self {
        Self { 
            map: HashMap::from_iter(vec![("update_clock".to_string(), Scheduler::TICK)])
        }
    }
}

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_default_resource::<InternalClock>();
        app.insert_system(*self.map.get("update_clock").unwrap(), update_clock);
    }

    fn phases_map(&mut self) -> &mut HashMap<String, f64> {
        &mut self.map
    }
}

pub fn update_clock(mut clock: ResMut<InternalClock>) {
    clock.update();
}
