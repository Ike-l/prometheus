use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::prelude::SupportedFloat;

use super::VertexDesc;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [SupportedFloat; 3],
    normal: [SupportedFloat; 3],
    colour: [SupportedFloat; 3],
    texture_coord: [SupportedFloat; 2],
}

impl Vertex {
    pub fn new<T, S, W, U>(position: T, normal: S, colour: W, texture_coord: U) -> Self
    where 
        T: Into<[SupportedFloat; 3]>,
        S: Into<[SupportedFloat; 3]>,
        W: Into<[SupportedFloat; 3]>,
        U: Into<[SupportedFloat; 2]>,
    {
        Self {
            position: position.into(),
            normal: normal.into(),
            colour: colour.into(),
            texture_coord: texture_coord.into()
        }
    }
}

pub const RAW_VERTEX_ROWS: u32 = 4;
impl VertexDesc for Vertex {
    fn desc() -> VertexBufferLayout<'static> {
        static ATTRIBUTES: [wgpu::VertexAttribute; RAW_VERTEX_ROWS as usize] = vertex_attr_array![
            0 => Float32x3, 
            1 => Float32x3, 
            2 => Float32x3,
            3 => Float32x2,
        ];

        VertexBufferLayout {
            array_stride: size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}
