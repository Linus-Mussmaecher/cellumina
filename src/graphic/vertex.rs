/// The number of vertices used in this program.
pub(super) const VERTICES_COUNT: usize = 4;

/// Vertices forming the corners of a rectangle
pub(super) const VERTICES: [Vertex; VERTICES_COUNT] = [
    Vertex {
        position: [-1., -1., 0.0],
        tex_coords: [0., 1.],
    },
    Vertex {
        position: [1., -1., 0.0],
        tex_coords: [1., 1.],
    },
    Vertex {
        position: [-1., 1., 0.0],
        tex_coords: [0., 0.],
    },
    Vertex {
        position: [1., 1., 0.0],
        tex_coords: [1., 0.],
    },
];

// indices to draw this rectangle of two triangles.
pub(super) const INDICES: &[u16] = &[0, 1, 2, 1, 3, 2];

/// A vertex in a vertex shader.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct Vertex {
    /// The vertex position in screen coordinates.
    pub(super) position: [f32; 3],
    /// The coordinates on the texture corresponding to this vertex.
    pub(super) tex_coords: [f32; 2],
}

impl Vertex {
    /// A representation of this struct's layout.
    pub(super) const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    /// A representation of a buffer containing multiple vertices.
    pub(super) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // how far are two elements in the buffer from each other
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // is an element in this buffer a single vertex or a single instance?
            step_mode: wgpu::VertexStepMode::Vertex,
            // what are the contents of this buffer?
            attributes: &Self::ATTRIBS,
        }
    }
}
