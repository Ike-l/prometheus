use std::collections::HashMap;

use hecs::Entity;
use small_read_only::ReadOnly;

#[derive(Debug, Default, ReadOnly)]
pub struct WorldRegistry {
    entity_to_label: HashMap<Entity, String>,
    label_to_entity: HashMap<String, Entity>
}

impl WorldRegistry {
    pub fn insert(&mut self, entity: Entity, label: String) -> (Option<String>, Option<Entity>) {
        (
            self.entity_to_label.insert(entity, label.clone()), 
            self.label_to_entity.insert(label, entity)
        )
    }

    pub fn remove_entity(&mut self, entity: &Entity) -> Result<String, ()> {
        let label = self.entity_to_label.remove(entity);
        if let Some(label) = label {
            self.label_to_entity.remove(&label);
            Ok(label)
        } else {
            Err(())
        }
    }

    pub fn remove_label(&mut self, label: &str) -> Result<Entity, ()> {
        let entity = self.label_to_entity.remove(label);
        if let Some(entity) = entity {
            self.entity_to_label.remove(&entity);
            Ok(entity)
        } else {
            Err(())
        }
    }
}
