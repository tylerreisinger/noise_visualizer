use glium;

use visualizer::Vertex;

pub struct Geometry<V, I>
where
    V: glium::Vertex,
    I: glium::index::Index,
{
    vertex_buffer: glium::VertexBuffer<V>,
    index_buffer: glium::IndexBuffer<I>,
}

impl<V, I> Geometry<V, I>
where
    V: glium::Vertex,
    I: glium::index::Index,
{
    pub fn new(
        vertex_buffer: glium::VertexBuffer<V>,
        index_buffer: glium::IndexBuffer<I>,
    ) -> Geometry<V, I> {
        Geometry {
            vertex_buffer,
            index_buffer,
        }
    }
}

pub fn create_cube() -> (Vec<Vertex>, Vec<u32>) {
    let vertices = vec![
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.0, 0.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [1.0, 0.0, 1.0],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.0, 1.0, 1.0],
            color: [0.0, 1.0, 1.0, 1.0],
        },
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, 0, 3, 4, 4, 3, 7, 7, 3, 6, 6, 3, 2, 2, 1, 6, 6, 1, 5, 5, 4, 0, 0, 1, 5,
        4, 5, 6, 6, 7, 4,
    ];

    (vertices, indices)
}
