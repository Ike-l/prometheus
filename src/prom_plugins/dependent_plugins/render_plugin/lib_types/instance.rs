use bytemuck::{Pod, Zeroable};
use cgmath::One;
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::{prelude::SupportedFloat, prom_plugins::dependent_plugins::interfaces::render_plugin::InstanceInterface};

use super::{VertexDesc, vertex::RAW_VERTEX_ROWS};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Instance {
    model: [[SupportedFloat; 4]; 4],
    normal: [[SupportedFloat; 4]; 4],
}

impl VertexDesc for Instance {
    fn desc() -> VertexBufferLayout<'static> {
        static ATTRIBUTES: [wgpu::VertexAttribute; 8] = vertex_attr_array![
            0+RAW_VERTEX_ROWS => Float32x4, 
            1+RAW_VERTEX_ROWS => Float32x4, 
            2+RAW_VERTEX_ROWS => Float32x4,
            3+RAW_VERTEX_ROWS => Float32x4,

            4+RAW_VERTEX_ROWS => Float32x4, 
            5+RAW_VERTEX_ROWS => Float32x4,
            6+RAW_VERTEX_ROWS => Float32x4,
            7+RAW_VERTEX_ROWS => Float32x4,
        ];

        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &ATTRIBUTES,
        }
    }
}

impl From<&Box<dyn InstanceInterface>> for Instance {
    fn from(value: &Box<dyn InstanceInterface>) -> Self {
        Self {
            model: value.model().into(),
            normal: value.normal().unwrap_or(cgmath::Matrix4::one()).into()
        }
    }
}
