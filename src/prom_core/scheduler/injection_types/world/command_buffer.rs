use small_derive_deref::{Deref, DerefMut};

use crate::prom_core::scheduler::{injection_types::resource::mut_referenced::ResMut, system::SystemParam, AccessMap, TypeMap};

/// provides complete freedom over a command buffer which is applied at the end of each tick
#[allow(missing_debug_implementations)]
#[derive(Deref, DerefMut)]
pub struct CommandBuffer<'a> {
    pub command_buffer: ResMut<'a, hecs::CommandBuffer>,
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