use std::any::type_name;

use crate::prom_core::app::App;

pub trait Plugin {
    fn build(&self, app: &mut App);
    fn id(&self) -> &'static str {
        type_name::<Self>()
    }
}
