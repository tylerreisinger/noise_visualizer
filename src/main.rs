#![allow(dead_code)]

extern crate cgmath;
extern crate game_time;
#[macro_use]
extern crate glium;
extern crate image;
extern crate noise_lib;
extern crate rand;

mod animated;
mod camera_controller;
mod render;
mod geom;
mod grid;
mod visualizer;
mod uniform;

use glium::glutin;
use cgmath::{Matrix4, Vector3};
use geom::GeometryProvider;

fn build_material_uniform(
    vis: &visualizer::Visualizer,
) -> glium::uniforms::UniformBuffer<visualizer::Materials> {
    let materials = visualizer::Materials::new(&[
        visualizer::Material {
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [1.0, 1.0, 1.0, 1.0],
            specular: [0.0, 0.0, 0.0, 1.0],
            shine: 120.0,
            _padding: Default::default(),
        },
        visualizer::Material {
            ambient: [0.2, 0.2, 0.2, 1.0],
            diffuse: [1.0, 1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0, 1.0],
            shine: 120.0,
            _padding: Default::default(),
        },
    ]);

    glium::uniforms::UniformBuffer::new(vis.display(), materials).unwrap()
}

fn build_geometry(
    vis: &visualizer::Visualizer,
) -> geom::Geometry<visualizer::Vertex, visualizer::Index> {
    let rng = rand::StdRng::new().unwrap();

    let noise = noise_lib::perlin::build_geometric_octaves(
        (2, 2),
        10,
        3.0,
        &mut noise_lib::perlin::RandomGradientBuilder2d::new(rng),
        &noise_lib::interpolate::ImprovedPerlinInterpolator::new(),
    );

    let grid = grid::make_noise_grid(&noise, (200, 200));
    let (vertices, indices) = grid.gen_vertex_buffer();
    let model = Matrix4::from_translation(Vector3::new(-50.0, -50.0, 20.0_f32))
        * Matrix4::from_nonuniform_scale(1.0, 1.0, 100.0)
        * Matrix4::from_scale(0.3333);

    let vertex_buffer = glium::VertexBuffer::new(vis.display(), &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        vis.display(),
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();

    geom::Geometry::new(vertex_buffer, index_buffer, model)
}

fn build_perlin_animation() -> Box<GeometryProvider<visualizer::Vertex, visualizer::Index>> {
    let rng = rand::StdRng::new().unwrap();

    let noise = noise_lib::perlin3d::build_geometric_octaves(
        (2, 2, 3),
        8,
        2.5,
        &mut noise_lib::perlin3d::RandomGradientBuilder3d::new(rng),
        &noise_lib::interpolate::ImprovedPerlinInterpolator::new(),
    );

    Box::new(animated::PerlinAnimation::new(noise, (150, 150)))
}

fn main() {
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Noise Visualizer")
        .with_dimensions(1024, 768);
    let mut vis = visualizer::Visualizer::new(window_builder);

    //let geom = build_geometry(&vis);
    let anim = build_perlin_animation();
    let materials = build_material_uniform(&vis);
    //vis.set_geometry(Box::new(geom));
    vis.set_geometry(anim);
    vis.set_materials(materials);

    vis.run();
}

fn build_z_reflection_matrix() -> Matrix4<f32> {
    Matrix4::new(
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
