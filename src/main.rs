#![allow(dead_code)]

extern crate cgmath;
extern crate game_time;
#[macro_use]
extern crate glium;
extern crate noise_lib;
extern crate rand;

mod camera_controller;
mod render;
mod geom;
mod grid;
mod visualizer;

use glium::glutin;
use rand::SeedableRng;
use cgmath::{Matrix4, Vector3};

fn build_geometry(
    vis: &visualizer::Visualizer,
) -> geom::Geometry<visualizer::Vertex, visualizer::Index> {
    let rng = rand::StdRng::new().unwrap();

    let noise = noise_lib::perlin::build_geometric_octaves(
        (1, 1),
        6,
        2.0,
        &mut noise_lib::perlin::RandomGradientBuilder2d::new(rng),
        &noise_lib::interpolate::ImprovedPerlinInterpolator::new(),
    );

    let grid = grid::make_noise_grid(&noise, (100, 100));
    let (vertices, indices) = grid.gen_vertex_buffer();
    let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 20.0_f32))
        * Matrix4::from_nonuniform_scale(1.0, 1.0, 25.0) * Matrix4::from_scale(1.0);

    let vertex_buffer = glium::VertexBuffer::new(vis.display(), &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        vis.display(),
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();

    geom::Geometry::new(vertex_buffer, index_buffer, model)
}

fn main() {
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Noise Visualizer")
        .with_dimensions(800, 600);
    let mut vis = visualizer::Visualizer::new(window_builder);

    let geom = build_geometry(&vis);
    vis.set_geometry(geom);

    vis.run();
}
