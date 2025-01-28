use std::{collections::HashSet, sync::Arc};

use small_derive_deref::{Deref, DerefMut};

#[derive(Debug, Deref, DerefMut)]
pub struct IterableRegistry<V> {
    register: HashSet<V>
}

#[derive(Debug, Deref, DerefMut)]
pub struct OwnedIterableRegistry<V> {
    register: HashSet<Arc<V>>
}
