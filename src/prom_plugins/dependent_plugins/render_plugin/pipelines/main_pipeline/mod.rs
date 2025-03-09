use wgpu::{
    include_wgsl, 
    CompareFunction, 
    DepthBiasState, 
    DepthStencilState, 
    Device, 
    FragmentState, 
    FrontFace, 
    MultisampleState, 
    PipelineLayoutDescriptor, 
    PrimitiveState, 
    PrimitiveTopology, 
    RenderPipeline, 
    RenderPipelineDescriptor, 
    StencilState, 
    TextureFormat, 
    VertexState
};

use crate::prom_plugins::dependent_plugins::render_plugin::{bind_groups::bind_group_layouts::BindGroupLayouts, lib_types::{
    instance::Instance, 
    vertex::Vertex, 
    VertexDesc
}};

pub fn create_main_pipeline(device: &Device, fragment_target_format: TextureFormat, bind_group_layouts: &BindGroupLayouts) -> RenderPipeline {
    let main_shader = device.create_shader_module(include_wgsl!("main.wgsl"));

    let vertex_layouts = [Vertex::desc(), Instance::desc()];
    
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Main"),
        layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layouts.main_layouts(),
            push_constant_ranges: &[]
        })),
        vertex: VertexState {
            module: &main_shader,
            entry_point: Some("main_vs"),
            buffers: &vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: &main_shader,
            entry_point: Some("main_fs"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: fragment_target_format,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::OVER,
                    color: wgpu::BlendComponent::OVER,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            front_face: FrontFace::Ccw,
 //           cull_mode: Some(Face::Back),
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        multiview: None,
        cache: None
    })
}
