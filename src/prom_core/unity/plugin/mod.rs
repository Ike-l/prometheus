use crate::prom_core::app::App;

use super::registry::identity::Identity;

pub trait Plugin {
    fn build(&self, app: &mut App);
    fn id(&self) -> Identity<String>;
}
