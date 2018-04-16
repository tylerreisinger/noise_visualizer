use grid;
use noise_lib;
use geom::{Geometry, GeometryProvider};
use cgmath::{Matrix4, Vector3};
use visualizer::{Index, Vertex, Visualizer};
use glium;

pub struct PerlinAnimation<N>
where
    N: noise_lib::noise::Noise<IndexType = Vector3<f64>>,
{
    noise: N,
    current_geom: Option<Geometry<Vertex, Index>>,
    current_frame: u32,
    dimensions: (u32, u32),
}

impl<N> PerlinAnimation<N>
where
    N: noise_lib::noise::Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)>,
{
    pub fn new(noise: N, dimensions: (u32, u32)) -> PerlinAnimation<N> {
        PerlinAnimation {
            noise,
            current_geom: None,
            current_frame: 0,
            dimensions,
        }
    }

    fn build_geometry(&mut self, vis: &Visualizer, z: f64) {
        let slice = noise_lib::slice::Slice2d::new(&self.noise, z);

        let grid = grid::make_noise_grid(&slice, self.dimensions);
        let (vertices, indices) = grid.gen_vertex_buffer();

        let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 20.0_f32))
            * Matrix4::from_nonuniform_scale(1.0, 1.0, (self.dimensions.0 as f32) / 2.0);

        let vertex_buffer = glium::VertexBuffer::new(vis.display(), &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            vis.display(),
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        let g = Geometry::new(vertex_buffer, index_buffer, model);
        self.current_geom = Some(g);
    }
}

impl<N> GeometryProvider<Vertex, Index> for PerlinAnimation<N>
where
    N: noise_lib::noise::Noise<IndexType = Vector3<f64>, DimType = (u32, u32, u32)>,
{
    fn get_geometry(&self) -> &Geometry<Vertex, Index> {
        self.current_geom.as_ref().unwrap()
    }

    fn update(&mut self, vis: &Visualizer) {
        self.current_frame += 1;
        let distance = (self.current_frame % 1000) as f64 / 1000.0;
        let z = if self.current_frame % 2000 >= 1000 {
            1.0 - distance
        } else {
            distance
        };

        self.build_geometry(vis, z);
    }
}
