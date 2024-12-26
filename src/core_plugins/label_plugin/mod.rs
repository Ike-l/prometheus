mod label;

use crate::prelude::*;

pub mod prelude {
    pub use super::{
        label::{
            LabelComponent, LabeledEntities
        },
        UPDATE_LABELS
    };
}

pub struct LabelPlugin;

pub const UPDATE_LABELS: f64 = 1.001;

impl PluginTrait for LabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_resource(label::LabeledEntities::default());
        
        app.add_system(UPDATE_LABELS, label::update_labeled_entities);
    }

    fn id(&self) -> PluginId {
        PluginId("prometheus_LabelPlugin")
    }
}

