use hecs::{Bundle, Component, DynamicBundle, Entity, World};
use small_derive_deref::{Deref, DerefMut};

use crate::prom_core::scheduler::{injection_types::resource::mut_referenced::ResMut, system::SystemParam, AccessMap, TypeMap};

/// provides complete freedom over a command buffer which is applied at the end of each tick
#[allow(missing_debug_implementations)]
#[derive(Deref, DerefMut)]
pub struct CommandBuffer<'a> {
    command_buffer: ResMut<'a, hecs::CommandBuffer>,
}

impl<'a> CommandBuffer<'a> {
    //pub fn clear(&mut self) { /* Allows me to get rid of access map */ }
    pub fn despawn(&mut self, entity: Entity) {
        self.value.despawn(entity);
    }
    pub fn insert(&mut self, entity: Entity, components: impl DynamicBundle) {
        self.value.insert(entity, components);          
    }
    pub fn insert_one(&mut self, entity: Entity, component: impl Component) {
        self.value.insert_one(entity, component);        
    }
    pub fn remove<T>(&mut self, ent: Entity) where T: Bundle + 'static {
        self.value.remove::<T>(ent);         
    }
    pub fn remove_one<T>(&mut self, ent: Entity) where T: Component {
        self.value.remove_one::<T>(ent);               
    }
    pub fn run_on(&mut self, world: &mut World) {
        self.value.run_on(world);           
    }
    pub fn spawn(&mut self, components: impl DynamicBundle) {
        self.value.spawn(components);           
    }
}

impl<'a> SystemParam for CommandBuffer<'a> {
    type Item<'new> = CommandBuffer<'new>;

    fn accesses(_access: &mut AccessMap) {}

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let value = Self::typed_mut_retrieve::<hecs::CommandBuffer>(resources);
        CommandBuffer {
            command_buffer: ResMut { value }
        }
    }
}