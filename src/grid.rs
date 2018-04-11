use std::f64;

use cgmath::{self, InnerSpace, Vector3};
use noise_lib;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

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

    pub fn from_vec(vec: Vec<f64>, width: u32, height: u32) -> Grid {
        assert!(width > 1 && height > 1);
        assert!((width * height) as usize == vec.len());
        Grid {
            width: width,
            height: height,
            vals: vec,
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
                    normal: [0.0, 0.0, 0.0],
                });
            }
        }

        for y in 0..(height - 1) {
            for x in 0..(width - 1) {
                let start_index = x + y * width;
                index_buffer.push(start_index);
                index_buffer.push(start_index + width);
                index_buffer.push(start_index + 1);

                let a1 = Vector3::from(vertex_buffer[start_index as usize].position);
                let a2 = Vector3::from(vertex_buffer[(start_index + width) as usize].position);
                let a3 = Vector3::from(vertex_buffer[(start_index + 1) as usize].position);
                let a4 = Vector3::from(vertex_buffer[(start_index + width + 1) as usize].position);

                let a1a2 = a2 - a1;
                let a1a3 = a3 - a1;

                let a2a4 = a4 - a2;
                let a2a3 = a3 - a2;
                let normal1 = a1a3.cross(a1a2).normalize();
                let normal2 = a2a3.cross(a2a4).normalize();

                vertex_buffer[start_index as usize].normal = normal1.into();
                vertex_buffer[(start_index + 1) as usize].normal =
                    ((normal1 + normal2) * 0.5).into();
                vertex_buffer[(start_index + width) as usize].normal =
                    ((normal1 + normal2) * 0.5).into();
                vertex_buffer[(start_index + width + 1) as usize].normal = normal2.into();

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

pub fn make_noise_grid<N: noise_lib::noise::Noise>(perlin: &N, dimensions: (u32, u32)) -> Grid {
    let (width, height) = dimensions;

    let dx = 1.0 / f64::from(width);
    let dy = 1.0 / f64::from(height);

    let (mut min, mut max) = (f64::MAX, f64::MIN);

    let mut grid_vec = Vec::with_capacity(width as usize * height as usize);

    for y in 0..height {
        let perlin_y = f64::from(y) * dy;
        for x in 0..width {
            let perlin_x = f64::from(x) * dx;

            let value = 0.5 + perlin.value_at(cgmath::Vector2::new(perlin_x, perlin_y));

            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }

            grid_vec.push(value);
        }
    }

    let scale = max - min;
    let coeff = 1.0 / scale;

    println!("{} {}", min, max);
    let scaled_vec = grid_vec.iter().map(|x| (x - min) * coeff).collect();
    Grid::from_vec(scaled_vec, width, height)
}
