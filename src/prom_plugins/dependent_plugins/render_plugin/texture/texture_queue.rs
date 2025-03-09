use std::path::PathBuf;

use crate::{prelude::{resources::Resources, Res, ResMut, WriteWorld}, prom_plugins::dependent_plugins::{interfaces::render_plugin::TextureInterface, render_plugin::window::state::WindowState}};

use super::Texture;

#[derive(Debug, Default)]
pub struct TextureQueue {
    queue: Vec<(PathBuf, String)>
}

impl TextureQueue {
    pub fn insert(&mut self, path: PathBuf, label: String) {
        self.queue.push((path, label));
    }

    pub fn drain(&mut self) -> Vec<(PathBuf, String)> {
        self.queue.drain(..).collect()
    }
}

pub fn textures_pre_process(
    state: Res<WindowState>,
    mut texture_queue: ResMut<TextureQueue>, 
    mut world: WriteWorld,
) {
    for (file_path, label) in texture_queue.drain() {
        let data = pollster::block_on(Resources::load_binary(&file_path)).expect("Resolving file path into binary");
        let texture = Texture::from_bytes(state.device(), state.queue(), &data).expect("Invalid texture");
        world.spawn((Box::new(texture) as Box<dyn TextureInterface>,), label);
    }
}
