use std::{any::{type_name, TypeId}, cell::{Ref, RefCell}};

use hecs::{DynamicBundle, Entity, World};

use crate::prom_core::{app::world_registry::WorldRegistry, scheduler::{injection_types::resource::mut_referenced::ResMut, system::SystemParam, Access, AccessMap, TypeMap}};

#[allow(missing_debug_implementations)]
pub enum Command {
    Despawn(Entity),
    Spawn(Entity, Option<String>)
}

pub type CommandQueue = Vec<Command>;

#[allow(missing_debug_implementations)]
pub struct WriteWorld<'a> {
    pub world_registry: RefCell<ResMut<'a, WorldRegistry>>,
    world: RefCell<ResMut<'a, World>>,
    command_queue: ResMut<'a, CommandQueue>,
}

impl<'a> WriteWorld<'a> {
    pub fn get_world(&self) -> Ref<ResMut<'a, World>> {
        self.world.borrow()
    }
    
    pub fn spawn<T, Y>(&mut self, bundle: T, label: Y)
    where 
        T: DynamicBundle + 'static,
        Option<String>: From<Y>, 
    {
        let entity = self.world.borrow_mut().spawn(bundle);
        let label: Option<String> = label.into();
        self.command_queue.push(Command::Spawn(entity, label.clone()));

        if let Some(label) = label {
            let (old_label, old_entity) = self.world_registry.borrow_mut().insert(entity, label);

            if let Some(l) = old_label {
                log::warn!("Label replaced: {l}");
            }

            if let Some(e) = old_entity {
                log::warn!("Entity replaced: {e:?}")
            }
        }
    }

    pub fn defered_despawn(&mut self, entity: Entity) {
        self.command_queue.push(Command::Despawn(entity));
    }

    pub fn read_history(&self) -> impl Iterator<Item = &Command> + '_ {
        self.command_queue.iter()        
    }

    pub fn queue_len(&self) -> usize {
        self.command_queue.len()
    }
}

impl<'a> SystemParam for WriteWorld<'a> {
    type Item<'new> = WriteWorld<'new>;

    fn accesses(access: &mut AccessMap) {
        if let Some(_) = access.insert(TypeId::of::<hecs::World>(), Access::Write) {
            panic!(
                "conflicting access in system; attempting to access {} twice, once mutably; consider creating a new phase. There is an implicit mutable borrow in WriteWorld", type_name::<hecs::World>()
            )
        }

        if let Some(_) = access.insert(TypeId::of::<WorldRegistry>(), Access::Write) {
            panic!(
                "conflicting access in system; attempting to access {} twice, once mutably; consider creating a new phase. There is an implicit mutable borrow in WriteWorld", type_name::<WorldRegistry>()
            )
        }
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let queue = Self::typed_mut_retrieve::<CommandQueue>(resources);
        let world = Self::typed_mut_retrieve::<hecs::World>(resources);
        let world_registry = Self::typed_mut_retrieve::<WorldRegistry>(resources);
        WriteWorld {
            command_queue: ResMut { value: queue },
            world: RefCell::new(ResMut { value: world }),
            world_registry: RefCell::new(ResMut { value: world_registry })
        }
    }
}