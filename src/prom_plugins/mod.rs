use crate::prelude::Plugin;

pub mod independent_plugins;
pub mod dependent_plugins;

fn box_plugin<T: Plugin + 'static>(plugin: T) -> Box<dyn Plugin> {
    Box::new(plugin)
}

pub fn prom_plugins() -> impl Iterator<Item = Box<dyn Plugin>> {
    vec![
        box_plugin(independent_plugins::clock_plugin::ClockPlugin::default()),
        box_plugin(dependent_plugins::render_plugin::RenderPlugin::default())
    ].into_iter()
}