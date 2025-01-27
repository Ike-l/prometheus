use std::{any::{type_name, TypeId}, ops::{Deref, DerefMut}};

use crate::prom_core::scheduler::{system::SystemParam, Access, AccessMap, TypeMap};

/// Mutable system param
#[derive(Debug)]
pub struct ResMut<'a, T: 'static> {
    pub value: &'a mut T
}

impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'res, T: 'static> SystemParam for ResMut<'res, T> {
    type Item<'new> = ResMut<'new, T>;

    fn accesses(access: &mut AccessMap) {
        assert!(
            access.insert(TypeId::of::<T>(), Access::Write).is_none(),
            "conflicting access in system; attempting to access {} twice with a mutable reference; consider creating a new phase", type_name::<T>()
        );        
    }

    unsafe fn retrieve(resources: &TypeMap) -> Self::Item<'_> {
        let value = Self::typed_mut_retrieve::<T>(resources);
        ResMut { value }
    }
}
