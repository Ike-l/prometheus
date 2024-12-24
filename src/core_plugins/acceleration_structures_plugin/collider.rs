use super::{
    aabb::AABB, 
    entity::prelude::InstanceRenderComponent, 
    label_plugin::prelude::LabelComponent, 
    object_plugin::prelude::ObjectRegistry, 
    promethius_std::prelude::Position, 
    RefWorld, Res
};

// AutoCollider => AABB Calculated automatically every tick based on the `Mesh` and `ModelMatrix`
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColliderComponent {
    pub model_bbox: Option<AABB>,
    pub bbox: Option<AABB>,
}

impl ColliderComponent {
    pub fn new(model_bbox: AABB) -> Self {
        Self { bbox: None, model_bbox: Some(model_bbox) }
    }

    pub fn length_x(&self) -> f64 {
        let bbox = self.bbox.as_ref().expect("No bbox");
        (bbox.max.x - bbox.min.x).abs()
    }

    pub fn length_y(&self) -> f64 {
        let bbox = self.bbox.as_ref().expect("No bbox");
        (bbox.max.y - bbox.min.y).abs()        
    }

    pub fn length_z(&self) -> f64 {
        let bbox = self.bbox.as_ref().expect("No bbox");
        (bbox.max.z - bbox.min.z).abs()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Collider {
    pub collider: ColliderComponent,
    pub entity_label: LabelComponent,
}

impl Collider {
    pub fn new(collider: ColliderComponent, entity_label: LabelComponent) -> Self {
        Self {
            collider,
            entity_label,
        }
    }
}

pub fn update_colliders(world: RefWorld, object_registry: Res<ObjectRegistry>) {
    let mut query = world.query::<(&InstanceRenderComponent, &mut ColliderComponent)>();
    for (_, (render, collider)) in &mut query {
        let model_aabb = match &collider.model_bbox {
            None => object_registry.objects.get(&render.object_label).unwrap_or_else(|| panic!("No object found with label: {:?}", &render.object_label)).aabb(),
            Some(model_aabb) =>  model_aabb,
        };

        let transform_position = |pos: &Position| {
            let transformed = render.model_vertex(cgmath::Vector4 {
                x: pos.x as f32,
                y: pos.y as f32,
                z: pos.z as f32,
                w: 1.0,
            });
            Position::new(transformed.x as f64, transformed.y as f64, transformed.z as f64)
        };

        let new_min = transform_position(&model_aabb.min);
        let new_max = transform_position(&model_aabb.max);

        let aabb = AABB::new(new_min, new_max);
        collider.bbox.replace(aabb);

        if collider.model_bbox.is_none() {
            collider.model_bbox.replace(model_aabb.clone());
        }
    }
}