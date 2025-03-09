use wgpu::VertexBufferLayout;

pub mod vertex;
pub mod instance;

pub mod camera;
pub mod material;
pub mod lights;

pub mod light_camera;

pub trait VertexDesc {
    fn desc() -> VertexBufferLayout<'static>;
}

