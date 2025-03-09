use std::collections::HashMap;

use hecs::Entity;

use crate::{prelude::{Command, WriteWorld}, prom_plugins::dependent_plugins::{interfaces::render_plugin::{CameraInterface, MaterialInterface, TextureInterface}, render_plugin::window::state::WindowState}};

use super::{texture_bind_group::TextureBindGroup, uniform_bind_group::UniformBindGroup};

#[derive(Debug, Default)]
pub struct BindGroupRegistries {
    pub textures: HashMap<Entity, TextureBindGroup>,
    pub materials: HashMap<Entity, UniformBindGroup>,
    pub cameras: HashMap<Entity, UniformBindGroup>,
}

pub fn update_bind_group_registries(world: &WriteWorld, bind_group_registries: &mut BindGroupRegistries, state: &WindowState) {
    for command in world.read_history() {
        match command {
            Command::Despawn(entity) => {
                let _ = bind_group_registries.textures.remove(entity);
                if let Some(mat) = bind_group_registries.materials.remove(entity) {
                    mat.buffer().destroy();
                }
                if let Some(cam) = bind_group_registries.cameras.remove(entity) {
                    cam.buffer().destroy();
                }
            },
            Command::Spawn(entity, label) => {
                let world = world.get_world();
                if let Some(texture) = world.get::<&Box<dyn TextureInterface>>(*entity).ok() {
                    println!("Creating texture!");
                    let texture = texture.as_ref();
                    bind_group_registries
                        .textures
                        .entry(*entity)
                        .or_insert(TextureBindGroup::new(
                            state.device(), 
                            state.bind_group_layouts()
                                .texture(), 
                            texture.view(), 
                            texture.sampler(), 
                            label.as_deref()
                        ));
                }

                if let Some(material) = world.get::<&Box<dyn MaterialInterface>>(*entity).ok() {
                    println!("Creating material!");
                    let material = material.as_ref();
                    bind_group_registries
                        .materials
                        .entry(*entity)
                        .or_insert(UniformBindGroup::new(
                            state.device(), 
                            state.bind_group_layouts()
                                .material(), 
                            &material.material().cast_slice(), 
                            label.as_deref()
                        ));
                }

                if let Some(camera) = world.get::<&Box<dyn CameraInterface>>(*entity).ok() {
                    println!("Creating camera!");
                    let camera = camera.as_ref();
                    bind_group_registries
                        .cameras
                        .entry(*entity)
                        .or_insert(UniformBindGroup::new(
                            state.device(), 
                            state.bind_group_layouts()
                                .camera(), 
                            &camera.camera().cast_slice(), 
                            label.as_deref()
                        ));
                };
            }
        }
    }
}

