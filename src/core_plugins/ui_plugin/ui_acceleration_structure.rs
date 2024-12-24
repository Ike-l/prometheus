use small_derive_deref::{
    Deref, DerefMut
};

use super::{
    acceleration_structures_plugin::prelude::{
        Collider, ColliderComponent, QuadTree
    }, 
    ui_component::UIComponent, 
    label_plugin::prelude::LabelComponent, 
    RefWorld, ResMut
};

#[derive(Debug, Default, Deref, DerefMut)]
pub struct UIAccelerationStructure {
    qt: QuadTree
}

pub fn create_acceleration_structure(acc_struct: ResMut<UIAccelerationStructure>, world: RefWorld) {
    let buffer = world.query::<(&ColliderComponent, &LabelComponent)>()
            .with::<&UIComponent>()
            .iter()
            .map(|(_, (collider, label))| {
                Collider::new(collider.clone(), label.clone())
    }).collect::<Vec<Collider>>();

    acc_struct.value.qt = QuadTree::auto(buffer);
}
