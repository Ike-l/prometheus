use crate::prelude::{
    *, 
    promethius_std::prelude::Position,
};

mod quad_tree;
mod collider;
mod aabb;

pub mod prelude {
    pub use super::{
        quad_tree::QuadTree, 
        AccelerationStructure,
        aabb::AABB,
        UPDATE_COLLIDERS,
        collider::{
            Collider, ColliderComponent
        }
    };    
}

pub struct AccelerationStructurePlugin;

pub const UPDATE_COLLIDERS: f64 = 1.001;

impl PluginTrait for AccelerationStructurePlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(UPDATE_COLLIDERS, collider::update_colliders);
    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_AccelerationStructurePlugin")
    }
}


pub trait AccelerationStructure {
    fn query(&self, position: &Position) -> Vec<collider::Collider>;
}

