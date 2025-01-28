use crate::prelude::Plugin;

pub mod clock_plugin;

fn box_plugin<T: Plugin + 'static>(plugin: T) -> Box<dyn Plugin> {
    Box::new(plugin)
}

pub fn prom_plugins() -> impl Iterator<Item = Box<dyn Plugin>> {
    vec![
        box_plugin(clock_plugin::ClockPlugin)
    ].into_iter()
}