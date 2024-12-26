mod object_registry;
mod models;

pub mod prelude {
    pub use super::*;
    
    pub use models::{
        material::*, 
        mesh::*,
        model::*,
    };
    
    pub use object_registry::{
        Object, ObjectRegistry
    };

    pub use super::UPDATE_REGISTRY;
}

use crate::prelude::*;

pub struct ObjectPlugin;

pub const UPDATE_REGISTRY: f64 = render_plugin::RENDER_SYSTEM - 0.001;

impl PluginTrait for ObjectPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(UPDATE_REGISTRY, object_registry::update_registry_instances);
        app.add_resource(object_registry::ObjectRegistry::default());

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_ObjectPlugin")
    }
}