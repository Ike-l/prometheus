use clock::Clock;
use small_derive_deref::{Deref, DerefMut};

use crate::{prelude::{App, Plugin, ResMut}, prom_core::scheduler::Scheduler};

pub mod clock;

#[derive(Debug, Default, Clone, Copy, Deref, DerefMut)]
pub struct InternalClock {
    clock: Clock
}

#[derive(Debug, Copy, Clone)]
pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_default_resource::<InternalClock>();
        app.insert_system(Scheduler::TICK, update_clock);
    }
}

pub fn update_clock(mut clock: ResMut<InternalClock>) {
    clock.update();
}
