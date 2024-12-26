mod render_component;
mod controller_component;
mod camera;
mod orthographic;
mod perspective;

pub mod prelude {
    use super::*;

    pub use render_component::*;

    pub use controller_component::*;

    pub use camera::Camera;

    pub use orthographic::{
        OrthoController, OrthoProjection, OrthoUniform, OrthoView
    };

    pub use perspective::{
        PerspController, PerspProjection, PerspUniform, PerspView
    };

    pub use super::{
        CAMERA_INPUT, UPDATE_CAMERA_BIND_GROUPS, UPDATE_CAMERA
    };
}


use crate::prelude::*;

pub struct CameraPlugin;

pub const UPDATE_CAMERA_BIND_GROUPS: f64 = render_plugin::RENDER_SYSTEM - 0.002;
pub const CAMERA_INPUT: f64 = render_plugin::RENDER_INPUT + 0.001;
pub const UPDATE_CAMERA: f64 = CAMERA_INPUT + 0.001;

impl PluginTrait for CameraPlugin {
    fn build(&self, app: &mut crate::app::App) {
        app.add_system(UPDATE_CAMERA_BIND_GROUPS, camera::update_camera_bind_group);
        app.add_system(CAMERA_INPUT, camera::input);
        app.add_system(UPDATE_CAMERA, camera::update_camera);

    }
    fn id(&self) -> PluginId {
        PluginId("prometheus_CameraPlugin")
    }
}


pub trait TransformComposer {
    #[rustfmt::skip]
    const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.5,
        0.0, 0.0, 0.0, 1.0,
    );
    fn compose_transform(&self) -> cgmath::Matrix4<f32>;
}
