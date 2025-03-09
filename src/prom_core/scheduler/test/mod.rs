#![cfg(test)]

use crate::prom_core::app::world_registry::WorldRegistry;

use super::{injection_types::world::command_queue::CommandQueue, Scheduler};

mod pass_tests;
mod panic_tests;

fn run_scheduler_tick(mut scheduler: Scheduler) -> Scheduler {
    scheduler.run(Scheduler::TICK, Scheduler::END);
    scheduler.run(Scheduler::TICK, Scheduler::END);
    scheduler.run(Scheduler::TICK, Scheduler::END);
 
    scheduler
}

fn run_scheduler_start(mut scheduler: Scheduler) -> Scheduler {
    scheduler.run(Scheduler::START, Scheduler::TICK);
    scheduler
}

fn create_scheduler() -> Scheduler {
    let mut scheduler = Scheduler::default();
    scheduler.insert_resource(hecs::World::new());
    scheduler.insert_resource(CommandQueue::new());
    scheduler.insert_resource(WorldRegistry::default());
    scheduler
}