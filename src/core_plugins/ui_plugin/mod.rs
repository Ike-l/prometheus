mod ui_component;
mod ui_acceleration_structure;
mod variants;

use crate::prelude::{
    *,
    promethius_std::prelude::Position,
};

use ui_acceleration_structure::{
    create_acceleration_structure, UIAccelerationStructure
};
use ui_component::input;

use small_derive_deref::{
    Deref, DerefMut
};

pub mod prelude {
    pub use super::{
        ui_acceleration_structure::UIAccelerationStructure,
        ui_component::{
            Event, UIComponent
        },
        variants::{
            timed_ui_component::{
                Delay, DelayedUIComponent
            },
            edged_ui_component::{
                Edge, EdgedUIComponent
            }
        }
    };
}

pub struct UIPlugin;

impl PluginTrait for UIPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(1.0011, create_acceleration_structure);
        app.add_system(1.0012, input);
        app.add_system(1.00115, variants::timed_ui_component::update_delayed_ui);
        app.add_system(1.00001, variants::edged_ui_component::store_lens);
        
        app.add_resource(UIAccelerationStructure::default());
        app.add_resource(CursorPosition::default());
    }
    
    fn id(&self) -> PluginId {
        PluginId("prometheus_UIPlugin")
    }
}

#[derive(Debug, Deref, DerefMut, Default)]
pub struct CursorPosition(Position);
