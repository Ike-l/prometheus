// can have a periodic-flushed-ui-component where it just stores a "every N ticks" or "every N duration" flush
use std::time::Duration;

use crate::prelude::{
    time_plugin::prelude::{
        Tick, Time
    }, 
    ui_plugin::prelude::UIComponent, 
    MutWorld, Res
};

#[derive(Debug)]
pub enum Delay {
    Time(Duration),
    Tick(Tick)
}

#[derive(Debug)]
pub struct DelayedUIComponent {
    pub delay_progress: Delay,
    delay_target: Delay,
}

impl DelayedUIComponent {
    pub fn new(delay_target: Delay) -> Self {
        let delay_progress = Self::origin_delay(&delay_target);
        Self { delay_progress, delay_target }
    }

    pub fn is_done(&self) -> bool {
        self.progress() >= 1.0
    }

    pub fn progress(&self) -> f64 {
        match &self.delay_progress {
            Delay::Tick(progress) => {
                match &self.delay_target {
                    Delay::Tick(target) => progress.0 as f64 / target.0 as f64,
                    _ => unreachable!()
                }
            },
            Delay::Time(progress) => {
                match self.delay_target {
                    Delay::Time(target) => progress.div_duration_f64(target),
                    _ => unreachable!()
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.delay_progress = Self::origin_delay(&self.delay_target);
    }

    fn origin_delay(delay_target: &Delay) -> Delay {
        match delay_target {
            Delay::Time(_) => Delay::Time(Duration::from_secs(0)),
            Delay::Tick(_) => Delay::Tick(Tick(0)),
        }
    }
}

pub fn update_delayed_ui(world: MutWorld, time: Res<Time>) {
    for (_, (ui, delay)) in &mut world.query::<(&mut UIComponent, &mut DelayedUIComponent)>() {
        // when no events it ensures the delay is reset.
        // when the first event is added it starts progress
        if ui.event_buffer.len() > 0 {
            match &mut delay.delay_progress {
                Delay::Tick(t) => t.0 += 1,
                Delay::Time(t) => *t += time.dt,
            }
    
            if delay.is_done() {
                ui.event_buffer.clear();
                delay.reset();
            }
        } else if delay.progress() != 0.0 {
            delay.reset();
        }        
    }
}