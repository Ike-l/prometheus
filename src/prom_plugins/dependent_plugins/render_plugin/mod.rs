use std::collections::HashMap;

use bind_groups::bind_group_registry::BindGroupRegistries;
use meshes::mesh_registry::MeshRegistry;
use render::render_scene;
use texture::texture_queue::{textures_pre_process, TextureQueue};
use window::{background_colour::BackgroundColour, state::instanciate_state};

use crate::prelude::{Plugin, Scheduler};

pub mod window;
pub mod lib_types;
pub mod pipelines;
pub mod render;
pub mod texture;
pub mod bind_groups;
pub mod meshes;

#[derive(Debug, Clone)]
pub struct RenderPlugin {
    map: HashMap<String, f64>
}

impl Default for RenderPlugin {
    fn default() -> Self {
        Self { 
            map: HashMap::from_iter(vec![
                ("render".to_string(), Scheduler::TICK + 0.5001), 
                ("texture_pre_process".to_string(), Scheduler::TICK + 0.3001),
            ])
        }
    }
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app.insert_system(Scheduler::START, instanciate_state);
        app.insert_default_resource::<BackgroundColour>();
        app.insert_default_resource::<TextureQueue>();
        app.insert_default_resource::<MeshRegistry>();
        app.insert_default_resource::<BindGroupRegistries>();
        // update mesh registry
        // render
        // change it to check command buffer once changed world and cmd buffer to query / write respectively
        // spawn_textures as well 
        // things like camera and material need updating before render anyways
        // then lastly there is shadow and lights
        // light is created each render from light interfaces
        // shadow is created once and stored in bind group registry? where?
        // skybox creates its camera and texture like main camera and texture (on spawn & init respectively)
        app.insert_system(*self.map.get("texture_pre_process").unwrap(), textures_pre_process);
        app.insert_system(*self.map.get("render").unwrap(), render_scene);
    }
    
    fn phases_map(&mut self) -> &mut std::collections::HashMap<String, f64> {
        &mut self.map
    }
}