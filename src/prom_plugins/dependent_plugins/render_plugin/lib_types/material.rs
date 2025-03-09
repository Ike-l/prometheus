use bytemuck::{Pod, Zeroable};

use crate::prelude::SupportedFloat;


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Material {
    pub ambience: [SupportedFloat; 4],
    pub specularity: [SupportedFloat; 4],
    pub diffusivity: [SupportedFloat; 4],
}

impl Default for Material {
    fn default() -> Self {
        Self {
            // opacity 1.0 so if used incorrectly it will show `Black` instead of seemingly not rendering - if used as return
            ambience: [0.0, 0.0, 0.0, 1.0],
            specularity: [0.0, 0.0, 0.0, 1.0],
            diffusivity: [0.0, 0.0, 0.0, 1.0]
        }
    }
}

impl Material {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}

