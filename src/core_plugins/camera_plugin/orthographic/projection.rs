use crate::prelude::{
    promethius_std::prelude::Position, 
    render_plugin::WindowDimensions
};

use super::super::TransformComposer;

#[derive(Debug)]
pub struct OrthoProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl OrthoProjection {
    pub fn new_square(length: f32, near: f32, far: f32) -> Self {
        Self::new_rect(length, length, near, far)
    }
    
    pub fn new_rect(width: f32, height: f32, near: f32, far: f32) -> Self {
        Self {
            left: -width / 2.,
            right: width / 2.,
            bottom: -height / 2.,
            top: height / 2.,
            near,
            far,
        }
    }

    pub fn to_world_space(&self, position: &Position, window_dimensions: &WindowDimensions) -> Position {
        let x_ndc = (2.0 * position.x / window_dimensions.width as f64) - 1.0;
        let y_ndc = 1.0 - (2.0 * position.y / window_dimensions.height as f64);

        let x_world = (self.right as f64 - self.left as f64) / 2.0 * x_ndc + (self.right as f64 + self.left as f64) / 2.0;
        let y_world = (self.top as f64 - self.bottom as f64) / 2.0 * y_ndc + (self.top as f64 + self.bottom as f64) / 2.0;

        Position::new(x_world, y_world, position.z)
    }
}

impl TransformComposer for OrthoProjection {
    fn compose_transform(&self) -> cgmath::Matrix4<f32> {
        Self::OPENGL_TO_WGPU_MATRIX * cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far)
    }
}