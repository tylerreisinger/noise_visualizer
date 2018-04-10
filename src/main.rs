#![allow(dead_code)]

extern crate cgmath;
extern crate game_time;
#[macro_use]
extern crate glium;
extern crate noise_lib;

mod camera_controller;
mod render;
mod grid;
mod visualizer;

use glium::glutin;

fn main() {
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Noise Visualizer")
        .with_dimensions(800, 600);
    let mut vis = visualizer::Visualizer::new(window_builder);
    vis.run();
}
