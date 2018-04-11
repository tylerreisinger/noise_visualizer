use glium;
use cgmath::Matrix4;

use visualizer;

#[derive(Clone, Copy, Debug, Default)]
pub struct CubeVertex {
    position: [f32; 3],
    color: [f32; 4],
}

pub struct Geometry<V, I>
where
    V: glium::Vertex,
    I: glium::index::Index,
{
    vertex_buffer: glium::VertexBuffer<V>,
    index_buffer: glium::IndexBuffer<I>,
    model: Matrix4<f32>,
}

impl<V, I> Geometry<V, I>
where
    V: glium::Vertex,
    I: glium::index::Index,
{
    pub fn new(
        vertex_buffer: glium::VertexBuffer<V>,
        index_buffer: glium::IndexBuffer<I>,
        model: Matrix4<f32>,
    ) -> Geometry<V, I> {
        Geometry {
            vertex_buffer,
            index_buffer,
            model,
        }
    }

    pub fn vertex_buffer(&self) -> &glium::VertexBuffer<V> {
        &self.vertex_buffer
    }
    pub fn index_buffer(&self) -> &glium::IndexBuffer<I> {
        &self.index_buffer
    }
    pub fn model(&self) -> &Matrix4<f32> {
        &self.model
    }
}

pub fn create_cube() -> (Vec<CubeVertex>, Vec<u32>) {
    let vertices = vec![
        CubeVertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        CubeVertex {
            position: [1.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        CubeVertex {
            position: [1.0, 1.0, 0.0],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        CubeVertex {
            position: [0.0, 1.0, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        CubeVertex {
            position: [0.0, 0.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        CubeVertex {
            position: [1.0, 0.0, 1.0],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        CubeVertex {
            position: [1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        CubeVertex {
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
