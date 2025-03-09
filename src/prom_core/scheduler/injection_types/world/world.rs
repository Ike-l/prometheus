use std::any::{type_name, TypeId};

use small_derive_deref::Deref;

use crate::prom_core::scheduler::{injection_types::resource::referenced::Res, system::SystemParam, Access, AccessMap, TypeMap};

/// Provides referential access to a world object, using hecs for now
#[allow(missing_debug_implementations)]
#[derive(Deref)]
pub struct ReadWorld<'a> {
    world: Res<'a, hecs::World>
}

impl<'a> SystemParam for ReadWorld<'a> {
    type Item<'new> = ReadWorld<'new>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<hecs::World>()).or_insert(Access::Read), Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase. There is an implicit borrow in WriteWorld", type_name::<hecs::World>(),
        );
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let value = Self::typed_retrieve::<hecs::World>(resources);
        ReadWorld { 
            world: Res { value }
        }
    }
}