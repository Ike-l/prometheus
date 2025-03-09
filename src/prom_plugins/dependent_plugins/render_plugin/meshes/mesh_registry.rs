use std::collections::HashMap;

use hecs::Entity;
use wgpu::Device;

use crate::{prelude::{Command, WriteWorld}, prom_plugins::dependent_plugins::{interfaces::render_plugin::MeshInterface, render_plugin::window::state::WindowState}};

use super::mesh::Mesh;

#[derive(Debug, Default)]
pub struct MeshRegistry {
    pub meshes: HashMap<Entity, Mesh>
}

impl MeshRegistry {
    pub fn create_mesh(&mut self, device: &Device, mesh: &Box<dyn MeshInterface>, entity: Entity, auto_insert: bool) -> Option<Mesh> {
        let mesh = Mesh::new(device, mesh);
        if auto_insert {
            self.meshes.insert(entity, mesh);
            None
        } else {
            Some(mesh)
        }
    }
}

pub fn update_mesh_registry(mesh_registry: &mut MeshRegistry, world: &WriteWorld, state: &WindowState) {
    for command in world.read_history() {
        match command {
            Command::Despawn(entity) => {
                if let Some(mesh) = mesh_registry.meshes.remove(&entity) {
                    mesh.vertex_buffer().destroy();
                    mesh.index_buffer().destroy();
                }
            },
            Command::Spawn(entity, _) => {
                let mesh = world.get_world();
                let mesh = mesh.query_one::<&Box<dyn MeshInterface>>(*entity);
                let mut mesh = mesh.unwrap();
                let mesh = mesh.get();
                if let Some(mesh) = mesh {
                    println!("Creating mesh!");
                    mesh_registry.create_mesh(state.device(), mesh, *entity, true);
                };
            }
        }
    }
}
