use small_read_only::ReadOnly;
use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, Device, SamplerBindingType, ShaderStages, TextureSampleType, TextureViewDimension};

#[derive(Debug, ReadOnly)]
pub struct BindGroupLayouts {
    lights: BindGroupLayout,
    shadow: BindGroupLayout,
    camera: BindGroupLayout,
    material: BindGroupLayout,
    texture: BindGroupLayout,
    bind_group_flags: BindGroupLayout,
    light_camera: BindGroupLayout,
} 

impl BindGroupLayouts {
    pub fn new(device: &Device) -> Self {
        let lights = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/Lights"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ]
        });

        let shadow = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/Shadow"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture { 
                        sample_type: TextureSampleType::Depth, 
                        view_dimension: TextureViewDimension::D2Array, 
                        multisampled: false 
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Comparison),
                    count: None
                },
            ],
        });

        let camera = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/Camera"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ]
        });

        let light_camera = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Light/Camera"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ]
        });

        let material = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/Material"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ],
        });

        let texture = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/Texture"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group_flags = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Main/BindGroupFlags"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: None 
                    },
                    count: None
                }
            ]
        });

        Self {
            lights,
            shadow,
            camera,
            material,
            texture,
            bind_group_flags,
            light_camera,
        }
    }


    pub fn main_layouts(&self) -> [&BindGroupLayout; 6] {
        [
            &self.lights,
            &self.shadow,
            &self.camera,
            &self.material,
            &self.texture,
            &self.bind_group_flags,
        ]
    }

    pub fn shadow_layouts(&self) -> [&BindGroupLayout; 1] {
        [
            &self.light_camera
        ]
    }
}
