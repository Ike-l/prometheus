use std::time::{Duration, Instant};

use small_read_only::ReadOnly;

#[derive(Debug, Copy, Clone, ReadOnly)]
pub struct Clock {
    start: Instant,
    current_tick_time: Instant,
    delta_tick_time: Duration,
    current_tick: u64,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            current_tick_time: Instant::now(),
            delta_tick_time: Duration::default(),
            current_tick: 0,
        }       
    }
}

impl Clock {
    pub fn update(&mut self) {
        self.delta_tick_time = Instant::now().duration_since(self.current_tick_time);
        self.current_tick_time += self.delta_tick_time;
        self.current_tick += 1;
    }

    pub fn rate(&self) -> f64 {
        let time = self.delta_tick_time.as_secs_f64();
        if time == 0.0 {
            0.0
        } else {
            1.0 / self.delta_tick_time.as_secs_f64()
        }
    }
}
