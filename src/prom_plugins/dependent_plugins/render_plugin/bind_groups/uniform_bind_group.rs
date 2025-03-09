use small_read_only::ReadOnly;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device, Queue};

#[derive(Debug, ReadOnly)]
pub struct UniformBindGroup {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl UniformBindGroup {
    pub fn new(device: &Device, layout: &BindGroupLayout, contents: &[u8], label: Option<&str>) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label,
            contents,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label,
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ],
        });

        Self {
            buffer, bind_group
        }
    }
    
    pub fn write(&self, queue: &Queue, data: &[u8]) {
        queue.write_buffer(&self.buffer, 0, data);
    } 

    pub fn take_buffer(self) -> Buffer {
        self.buffer
    }
}