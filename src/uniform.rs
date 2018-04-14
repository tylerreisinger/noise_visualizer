use glium::{self, uniforms};

pub struct UniformBlock<U: glium::buffer::Content + uniforms::UniformBlock> {
    values: Vec<U>,
    buffer: Option<uniforms::UniformBuffer<U>>,
}

impl<U> UniformBlock<U>
where
    U: glium::buffer::Content + uniforms::UniformBlock,
{
    pub fn new(data: Vec<U>) -> UniformBlock<U> {
        UniformBlock {
            values: data,
            buffer: None,
        }
    }
}
