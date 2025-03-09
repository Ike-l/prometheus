use cgmath::{Array, Vector3};
use small_read_only::ReadOnly;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, Buffer, BufferUsages, Device};

use crate::prom_plugins::dependent_plugins::{interfaces::render_plugin::MeshInterface, render_plugin::lib_types::vertex::Vertex};

#[derive(Debug, ReadOnly)]
pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    indices: u32,

    pub material_id: Option<String>,
    pub texture_id: Option<String>,
    pub object_id: Option<String>,
}

impl Mesh {
    fn from_data(
        device: &Device,
        vertices: &Vec<Vertex>,
        indices: &Vec<u32>,
        material_id: Option<String>,
        texture_id: Option<String>,
        object_id: Option<String>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer, 
            index_buffer, 
            indices: indices.len() as u32, 
            material_id, 
            texture_id,
            object_id,
        }
    }

    pub(crate) fn new(device: &Device, mesh: &Box<dyn MeshInterface>) -> Self {
        let positions = mesh.positions();

        let normals = if let Some(normals) = mesh.normals() {
            normals
        } else {
            &(0..positions.len()).into_iter().map(|_| Vector3::from_value(0.0)).collect()
        };

        let indices = mesh.indices();
        
        let mut vertices = Vec::with_capacity(positions.len());
        
        if let Some(colours) = mesh.colours() {
            positions.iter().zip(normals.iter()).zip(colours.iter()).for_each(|((pos, norm), col)| {
                vertices.push(Vertex::new(*pos, *norm, *col, [0.0, 0.0]));
            });
        } else {
            let texture_coords = mesh.texture_coords().expect("Mesh must have either colours or texture coordinates.");
            positions.iter().zip(normals.iter()).zip(texture_coords.iter()).for_each(|((pos, norm), tex)| {
                vertices.push(Vertex::new(*pos, *norm, [0.0, 0.0, 0.0], *tex));
            });
        }
        

        Self::from_data(
            device,
            &vertices,
            &indices,
            mesh.material_id().map(ToOwned::to_owned),
            mesh.texture_id().map(ToOwned::to_owned),
            mesh.object_id().map(ToOwned::to_owned),
        )
    }
}