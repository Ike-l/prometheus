use bytemuck::{Pod, Zeroable};

use crate::prelude::SupportedFloat;


#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct Camera {
    pub view_projection: [[SupportedFloat; 4]; 4],
    pub inverse_projection: [[SupportedFloat; 4]; 4],
    pub position: [SupportedFloat; 3],
    pub padding: SupportedFloat,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            // Identity
            view_projection: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            inverse_projection: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            padding: 0.0
        }
    }    
}

impl Camera {
    pub fn cast_slice(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[*self]).to_vec()
    }
}
