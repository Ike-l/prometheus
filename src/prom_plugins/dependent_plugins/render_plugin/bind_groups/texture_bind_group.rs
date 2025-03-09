use small_read_only::ReadOnly;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device, Sampler, TextureView};

#[derive(Debug, ReadOnly)]
pub struct TextureBindGroup {
    bind_group: BindGroup
}

impl TextureBindGroup {
    pub fn new(device: &Device, layout: &BindGroupLayout, texture_view: &TextureView, texture_sampler: &Sampler, label: Option<&str>) -> Self {
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label,
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view)
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(texture_sampler)
                },
            ],
        });

        Self {
            bind_group
        }
    }
}

