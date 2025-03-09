pub mod texture_queue;

use wgpu::{AddressMode, CompareFunction, Device, Extent3d, FilterMode, Sampler, SamplerDescriptor, SurfaceConfiguration, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDimension};

use crate::prom_plugins::dependent_plugins::interfaces::render_plugin::TextureInterface;

#[derive(Debug)]
pub struct Texture {
    view: TextureView,
    sampler: Sampler,
}

impl Texture {
    pub fn new(view: TextureView, sampler: Sampler) -> Self {
        Self {
            view,
            sampler
        }
    }

    pub fn create_depth_texture(device: &Device, config: &SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[]
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None, 
            address_mode_u: AddressMode::ClampToEdge, 
            address_mode_v: AddressMode::ClampToEdge, 
            address_mode_w: AddressMode::ClampToEdge, 
            mag_filter: FilterMode::Linear, 
            min_filter: FilterMode::Linear, 
            mipmap_filter: FilterMode::Nearest, 
            lod_min_clamp: 0.0, 
            lod_max_clamp: 100.0, 
            compare: Some(CompareFunction::LessEqual), 
            ..Default::default()
        });

        Self {
            view, 
            sampler, 
        }
    }

    pub fn from_image(device: &wgpu::Device, queue: &wgpu::Queue, img: &image::DynamicImage) -> anyhow::Result<Self> {
        let size = Extent3d {
            width: img.width(),
            height: img.height(),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[]
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge, 
            address_mode_v: AddressMode::ClampToEdge, 
            address_mode_w: AddressMode::ClampToEdge, 
            mag_filter: FilterMode::Linear, 
            min_filter: FilterMode::Nearest, 
            mipmap_filter: FilterMode::Nearest, 
            lod_min_clamp: 0.0, 
            lod_max_clamp: 100.0, 
            compare: None, 
            ..Default::default()
        });

        let dimensions = image::GenericImageView::dimensions(img);
        queue.write_texture(
            TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &img.to_rgba8(),
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        Ok(Self {
            view, 
            sampler, 
        })
    }

    pub fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8]) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img)
    }

    pub fn dummy(device: &Device, format: TextureFormat, dimension: Option<TextureViewDimension>, compare: Option<CompareFunction>) -> Self {
        let size = Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dummy Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension,
            ..Default::default()
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            compare,
            ..Default::default()
        });

        Self {
            view,
            sampler
        }
    }

    pub fn dummy_default(device: &Device) -> Self {
        Self::dummy(device, TextureFormat::Rgba8UnormSrgb, None, None)
    }

    pub fn dummy_depth(device: &Device) -> Self {
        Self::dummy(device, TextureFormat::Depth32Float, None, None)
    }

    pub fn dummy_depth_array(device: &Device) -> Self {
        Self::dummy(device, TextureFormat::Depth32Float, Some(TextureViewDimension::D2Array), None)
    }

    pub fn dummy_depth_array_cmp(device: &Device) -> Self {
        Self::dummy(device, TextureFormat::Depth32Float, Some(TextureViewDimension::D2Array), Some(CompareFunction::Less))
    }
}

impl TextureInterface for Texture {
    fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    fn view(&self) -> &TextureView {
        &self.view
    }
}
