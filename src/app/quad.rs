use wgpu::util::DeviceExt;

use super::vertex::Vertex;
pub struct Quad {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'static>,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
        uv: [0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

impl Quad {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: 6,
            vertex_buffer_layout: Vertex::desc(),
        }
    }
}
