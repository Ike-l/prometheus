use std::{collections::HashMap, sync::Arc};

use small_derive_deref::{Deref, DerefMut};

use super::identity::Identity;

#[derive(Debug, Deref, DerefMut)]
pub struct LookupRegistry<K, V> {
    register: HashMap<Identity<K>, V>
}

#[derive(Debug, Deref, DerefMut)]
pub struct OwnedLookupRegistry<K, V> {
    register: HashMap<Identity<K>, Arc<V>>
}
