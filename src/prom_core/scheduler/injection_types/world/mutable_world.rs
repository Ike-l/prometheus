use std::any::{type_name, TypeId};

use small_derive_deref::{Deref, DerefMut};

use crate::prom_core::scheduler::{injection_types::resource::mut_referenced::ResMut, system::SystemParam, Access, AccessMap, TypeMap};

/// Provides mutable referential access to a world object, using hecs for now
#[allow(missing_debug_implementations)]
#[derive(Deref, DerefMut)]
pub struct MutWorld<'a> {
    world: ResMut<'a, hecs::World>,
}

impl<'a> SystemParam for MutWorld<'a> {
    type Item<'new> = MutWorld<'new>;

    fn accesses(access: &mut AccessMap) {
        if let Some(_) = access.insert(TypeId::of::<hecs::World>(), Access::Write) {
            panic!(
                "conflicting access in system; attempting to access {} mutably twice; consider creating a new phase", type_name::<hecs::World>()
            )
        }
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let value = Self::typed_mut_retrieve::<hecs::World>(resources);
        MutWorld { 
            world: ResMut { value }
        }
    }
}