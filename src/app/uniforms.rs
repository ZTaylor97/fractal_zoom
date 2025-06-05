use wgpu::{BindGroupLayout, util::DeviceExt};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformData {
    pub time: f32,
    pub zoom: f32,
    pub offset: [f32; 2],
}

pub struct Uniforms {
    pub uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    uniform_data: UniformData,
}

impl UniformData {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            zoom: 1.0,
            offset: [0.0, 0.0],
        }
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

impl Uniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let uniform_data = UniformData::new();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &UniformData::bind_group_layout(&device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            uniform_bind_group,
            uniform_buffer,
            uniform_data,
        }
    }

    pub fn update(&mut self, queue: &mut wgpu::Queue, time: f32, zoom: f32, offset: [f32; 2]) {
        self.uniform_data = UniformData { time, zoom, offset };
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.uniform_data),
        );
    }
}
