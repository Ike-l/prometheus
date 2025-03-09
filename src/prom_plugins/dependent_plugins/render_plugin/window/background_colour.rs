use cgmath::Vector4;

use crate::prelude::SupportedFloat;

#[derive(Debug, Copy, Clone)]
pub struct BackgroundColour {
    colour: Vector4<SupportedFloat>
}

impl Default for BackgroundColour {
    fn default() -> Self {
        Self {
            colour: Vector4 { x: 0.1, y: 0.2, z: 0.3, w: 1.0 }
        }
    }
}

impl From<&BackgroundColour> for wgpu::Color {
    fn from(c: &BackgroundColour) -> Self {
        Self {
            r: c.colour.x as f64,
            g: c.colour.y as f64,
            b: c.colour.z as f64,
            a: c.colour.w as f64,
        }
    }
}
