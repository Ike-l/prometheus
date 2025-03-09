use bytemuck::{Pod, Zeroable};

use crate::prelude::SupportedFloat;


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LightCamera {
    pub view_projection: [[SupportedFloat; 4]; 4],
}

impl LightCamera {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}
