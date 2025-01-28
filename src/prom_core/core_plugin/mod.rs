use winit::window::{Window, WindowId};

use crate::prelude::Res;

use super::unity::registry::lookup_registry::OwnedLookupRegistry;

pub fn loop_start(registry: Res<OwnedLookupRegistry<WindowId, Window>>) {
    for (id, window) in registry.iter() {
        log::info!("Redraw on window_id: {id:?}");
        window.as_ref().request_redraw();
    }
}
