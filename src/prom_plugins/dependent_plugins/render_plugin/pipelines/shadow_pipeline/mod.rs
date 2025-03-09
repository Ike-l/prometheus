use wgpu::{include_wgsl, CompareFunction, DepthBiasState, DepthStencilState, Device, Features, FrontFace, MultisampleState, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, StencilState, TextureFormat, VertexState};

use crate::prom_plugins::dependent_plugins::render_plugin::{bind_groups::bind_group_layouts::BindGroupLayouts, lib_types::{instance::Instance, vertex::Vertex, VertexDesc}};


pub fn create_shadow_pipeline(device: &Device, bind_group_layouts: &BindGroupLayouts) -> RenderPipeline {
    let shader = device.create_shader_module(include_wgsl!("shadow.wgsl"));

    let vertex_layouts = [Vertex::desc(), Instance::desc()];
    
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Shadow"),
        layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layouts.shadow_layouts(),
            push_constant_ranges: &[]
        })),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("main"),
            buffers: &vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: None,
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Ccw,
 //           cull_mode: Some(Face::Back),
            cull_mode: None,
            unclipped_depth: device.features().contains(Features::DEPTH_CLIP_CONTROL),
            ..Default::default()
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState::default(),
            bias: DepthBiasState {
                constant: 2,
                slope_scale: 2.0,
                clamp: 0.0
            },
        }),
        multisample: MultisampleState::default(),
        multiview: None,
        cache: None
    })
}