mod button;
mod ui_acceleration_structure;

pub mod prelude {
    pub use super::button::{
        ToggleUIComponent, DelayedUIComponent, Delay
    };
}

use ui_acceleration_structure::create_acceleration_structure;

use crate::prelude::*;

pub struct UIPlugin;

impl PluginTrait for UIPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.001, create_acceleration_structure);
        app.add_system(1.002, input);

        app.add_resource(ui_acceleration_structure::UIAccelerationStructure::default());
    }

    fn id(&self) -> PluginId {
        PluginId("prometheus_UIPlugin")
    }
}

pub fn input(
    window_events: EventReader<WindowEventBus>, 
    device_events: EventReader<DeviceEventBus>, 
    world: MutWorld
) {
    // check event if a location is specified:
    // --> find the component using QT and update its state.
    // --> else do nothing
}
