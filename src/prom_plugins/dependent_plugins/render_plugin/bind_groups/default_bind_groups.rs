use small_read_only::ReadOnly;
use wgpu::{BindGroup, Device, Sampler, TextureView, TextureViewDimension};

use crate::prom_plugins::dependent_plugins::{interfaces::render_plugin::TextureInterface, render_plugin::{lib_types::{camera::Camera, material::Material}, texture::Texture}};

use super::{bind_group_layouts::BindGroupLayouts, texture_bind_group::TextureBindGroup, uniform_bind_group::UniformBindGroup};

#[derive(Debug, ReadOnly)]
pub struct DefaultBindGroups {
    camera: BindGroup,
    material: BindGroup,
    texture: BindGroup,
    shadow: (BindGroup, wgpu::Texture),
}

const MAX_LIGHTS: u32 = 256;

impl DefaultBindGroups {
    pub fn new(device: &Device, layouts: &BindGroupLayouts) -> Self {
        let camera = UniformBindGroup::new(device, layouts.camera(), &Camera::default().cast_slice(), Some("Default Bind Group")).bind_group().clone();
        let material = UniformBindGroup::new(device, layouts.material(), &Material::default().cast_slice(), Some("Default Bind Group")).bind_group().clone();
        
        let (shadow_view, shadow_sampler, shadow_texture) = create_shadow(device, MAX_LIGHTS);
        let shadow = TextureBindGroup::new(device, layouts.shadow(), &shadow_view, &shadow_sampler, Some("Shadow Bind Group")).bind_group().clone();

        let dummy_texture = Texture::dummy_default(device);
        let texture = TextureBindGroup::new(device, layouts.texture(), &dummy_texture.view(), &dummy_texture.sampler(), Some("Default Bind Group")).bind_group().clone();
        Self {
            camera,
            material,
            texture,
            shadow: (shadow, shadow_texture)
        }
    }
}

pub fn create_shadow(device: &Device, layers: u32) -> (TextureView, Sampler, wgpu::Texture) {
    let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Shadow"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        compare: Some(wgpu::CompareFunction::LessEqual),
        ..Default::default()
    });

    let shadow_texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: 1024,
            height: 1024,
            //depth_or_array_layers: (lights.len() as u32).max(1),
            depth_or_array_layers: layers,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        label: None,
        view_formats: &[],
    });

    let shadow_view = shadow_texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..Default::default()
    });

    (shadow_view, shadow_sampler, shadow_texture)
}