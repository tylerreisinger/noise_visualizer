#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

#[derive(Clone, Debug)]
pub struct Grid {
    vals: Vec<f64>,
    width: u32,
    height: u32,
}

pub type Index = u32;

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        assert!(width > 1 && height > 1);
        Grid {
            width: width,
            height: height,
            vals: vec![0.0; (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn gen_vertex_buffer(&self) -> (Vec<Vertex>, Vec<Index>) {
        let (width, height) = (self.width, self.height);
        let num_vals = (width as usize) * (height as usize);
        let mut vertex_buffer = Vec::with_capacity(num_vals);
        let mut index_buffer: Vec<Index> =
            Vec::with_capacity((width as usize - 1) * (height as usize - 1) * 6);

        for y in 0..height {
            for x in 0..width {
                let index = (x + y * width) as usize;
                vertex_buffer.push(Vertex {
                    position: [x as f32, y as f32, self.vals[index] as f32],
                });
            }
        }

        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                let start_index = x + y * width;
                index_buffer.push(start_index);
                index_buffer.push(start_index + width);
                index_buffer.push(start_index + 1);

                index_buffer.push(start_index + width);
                index_buffer.push(start_index + width + 1);
                index_buffer.push(start_index + 1);
            }
        }

        (vertex_buffer, index_buffer)
    }
}

impl AsRef<[f64]> for Grid {
    fn as_ref(&self) -> &[f64] {
        self.vals.as_slice()
    }
}
impl AsMut<[f64]> for Grid {
    fn as_mut(&mut self) -> &mut [f64] {
        self.vals.as_mut_slice()
    }
}
