mod raw_render_component;
mod render_component;
mod render_components_tests;

// Can be f32 | f16 due to shader limitation
pub type FloatPrecision = f32;
pub const VERTEX_FORMAT: wgpu::VertexFormat = wgpu::VertexFormat::Float32x4;

pub mod prelude {
    pub use super::{
        raw_render_component::RawRenderComponent,
        render_component::InstanceRenderComponent
    };
}
