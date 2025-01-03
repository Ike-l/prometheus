use std::time::Duration;

use super::{prelude::Accumulators, time::Time, Res, ResMut};

pub fn create_fps_counter(interval: Duration, alt_name: Option<&str>) -> impl Fn(Res<Time>, ResMut<Accumulators>) {
    let accumulator_name = alt_name.unwrap_or("fps_counter").to_owned();  

    move |time: Res<Time>, mut accumulators: ResMut<Accumulators>| {
        let acc = match accumulators.get_mut(&accumulator_name) {
            Some(acc) => acc,
            None => &mut accumulators.insert_one(&accumulator_name).expect("cannot create `fps_counter` accumulator"),
        };
        
        if acc.time_since() >= interval {
            println!("FPS: {:?}", time.fps());
            acc.update();
        }
    }
}
