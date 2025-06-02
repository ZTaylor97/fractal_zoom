use wgpu::{BindGroupLayout, util::DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub time: f32,
    pub zoom: f32,
    pub offset: [f32; 2],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            zoom: 1.0,
            offset: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, t: f32) {
        self.time = t;
    }

    pub fn create_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(self),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn write_to_gpu(&self, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
        queue.write_buffer(buffer, 0, bytemuck::bytes_of(self));
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT, // or VERTEX | FRAGMENT
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        })
    }
}
