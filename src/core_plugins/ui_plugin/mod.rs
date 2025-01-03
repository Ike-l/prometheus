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
        CREATE_AS, UI_INPUT, UPDATE_DELAYED_UI, STORE_BUFFER_LENGTHS,
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

// formula for dynamic UI components:
// let [min/max] = self_[min/max].[x/y/z] [+/-] self_[width/height/depth] * scale * (normalised [width/height/depth] direction.[x/y/z])
// instance.set_[min/min_max](^...)
//
// where normalised direction: instance.normalised yada yada

pub struct UIPlugin;

pub const CREATE_AS: f64 = acceleration_structures_plugin::UPDATE_COLLIDERS + 0.000_1;
pub const UI_INPUT: f64 = CREATE_AS + 0.000_1;
pub const UPDATE_DELAYED_UI: f64 = CREATE_AS + 0.000_05;
pub const STORE_BUFFER_LENGTHS: f64 = 1.00001;

impl PluginTrait for UIPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(CREATE_AS, create_acceleration_structure);
        app.add_system(UI_INPUT, input);
        app.add_system(UPDATE_DELAYED_UI, variants::timed_ui_component::update_delayed_ui);
        app.add_system(STORE_BUFFER_LENGTHS, variants::edged_ui_component::store_lens);
        
        app.add_resource(UIAccelerationStructure::default());
        app.add_resource(CursorPosition::default());
    }
    
    fn id(&self) -> PluginId {
        PluginId("prometheus_UIPlugin")
    }
}

#[derive(Debug, Deref, DerefMut, Default)]
pub struct CursorPosition(Position);
