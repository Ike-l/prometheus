use std::{any::type_name, collections::HashMap};

use crate::prom_core::app::App;

pub trait Plugin {
    fn build(&self, app: &mut App);
    fn phases_map(&mut self) -> &mut HashMap<String, f64>;
    fn id(&self) -> &'static str {
        type_name::<Self>()
    }
}
