use std::{any::{type_name, TypeId}, ops::Deref};

use crate::prom_core::scheduler::{system::SystemParam, Access, AccessMap, TypeMap};

/// immutable system param
#[derive(Debug)]
pub struct Res<'a, T: 'static> {
    pub value: &'a T
}

impl <T: 'static> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn accesses(access: &mut AccessMap) {
        assert_eq!(
            *access.entry(TypeId::of::<T>()).or_insert(Access::Read), Access::Read,
            "conflicting access in system; attempting to access {} mutably and immutably at the same time; consider creating a new phase", type_name::<T>(),
        );
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let value = Self::typed_retrieve::<T>(resources);
        Res { value }
    }
}
