use small_read_only::ReadOnly;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Buffer, BufferBinding, BufferUsages, Device};

#[derive(Debug, ReadOnly)]
pub struct StorageBindGroup {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl StorageBindGroup {
    pub fn new(device: &Device, layout: &BindGroupLayout, stored: &[u8], label: Option<&str>) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents: &stored,
            usage: BufferUsages::STORAGE
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label,
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None
                }),
            }],
        });

        Self {
            buffer,
            bind_group
        }
    }

    pub fn take_buffer(self) -> Buffer {
        self.buffer
    }
}