use std::ops::Deref;
use std::f32;
use std::cell::RefCell;

use glium::glutin;
use glium::{self, Surface};
use cgmath::{self, InnerSpace, Matrix3, Matrix4, Vector3};
use noise_lib;
use rand::{self, SeedableRng};

use camera_controller::CameraController;
use geom;
use grid;

#[derive(Copy, Clone)]
struct Lights {
    dir: [f32; 3],
}
implement_uniform_block!(Lights, dir);

#[derive(Copy, Clone)]
struct Materials {
    ambient: [f32; 4],
    diffuse: [f32; 4],
    specular: [f32; 4],
    shine: f32,
}
implement_uniform_block!(Materials, ambient, diffuse, specular, shine);

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

pub struct Visualizer {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    running: bool,
    shader_program: glium::Program,
    camera_controller: RefCell<CameraController>,
}

impl Visualizer {
    pub fn new(window_builder: glutin::WindowBuilder) -> Visualizer {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();
        let camera_controller = CameraController::new(
            Vector3::new(5.0, 5.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            f32::consts::PI / 2.0,
            f32::consts::PI / 2.0,
        );

        let display = glium::Display::new(window_builder, context, &events_loop).unwrap();

        let shader_program = program!(&display,
            330 => {
                vertex: include_str!("glsl/lighting_vert.glsl"),
                fragment: include_str!("glsl/lighting_frag.glsl"),
        }).unwrap();

        Visualizer {
            events_loop,
            display: display,
            running: true,
            shader_program,
            camera_controller: RefCell::new(camera_controller),
        }
    }

    pub fn display(&self) -> &glium::Display {
        &self.display
    }
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    pub fn run(&mut self) {
        let mut running = self.running;

        while running {
            let mut target = self.display().draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            self.event_loop();
            self.update();
            self.draw(&mut target);

            target.finish().unwrap();
            running = self.running;
        }
    }

    fn event_loop(&mut self) {
        let mut is_closing = false;
        let mut camera_controller = self.camera_controller.borrow_mut();
        self.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => is_closing = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    camera_controller.handle_keyboard_input(&input);
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (dx, dy) = (position.0 - 400.0, position.1 - 300.0);
                    if dx.abs() > 100.0 || dy.abs() > 100.0 {

                    } else {
                        camera_controller.handle_mouse_move((dx, dy));
                    }
                }
                glutin::WindowEvent::MouseWheel { delta, .. } => {
                    camera_controller.handle_mouse_wheel(&delta);
                }
                _ => (),
            },
            _ => (),
        });
        self.display
            .gl_window()
            .deref()
            .set_cursor_position(400, 300)
            .unwrap();

        self.running = !is_closing;
    }
    fn update(&self) {}
    fn draw(&self, target: &mut glium::Frame) {
        let reflect = Matrix4::new(
            -1.0,
            0.0,
            0.0,
            0.0,
            0.0,
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
            1.0,
        );
        let (mut view, perspective) = self.camera_controller
            .borrow()
            .make_view_perspective_matrix(4.0 / 3.0, 5.0, 1000.0);

        view = reflect * view;

        /*let view_projection = reflect
            * self.camera_controller
                .borrow()
                .make_view_perspective_matrix(4.0 / 3.0, 5.0, 1000.0);
        */

        let rng = rand::StdRng::from_seed(&[0; 1]);

        let noise = noise_lib::perlin::build_geometric_octaves(
            (2, 2),
            6,
            2.0,
            &mut noise_lib::perlin::RandomGradientBuilder2d::new(rng),
            &noise_lib::interpolate::ImprovedPerlinInterpolator::new(),
        );

        let perlin = noise_lib::perlin::Perlin::new(
            (3, 3),
            &mut noise_lib::perlin::RandomGradientBuilder2d::new(rng),
            noise_lib::interpolate::ImprovedPerlinInterpolator::new(),
        );

        let light_uniforms = glium::uniforms::UniformBuffer::new(
            self.display(),
            Lights {
                dir: cgmath::conv::array3(Vector3::new(0.0, 2.0, 1.0_f32).normalize()),
            },
        ).unwrap();
        let material_uniforms = glium::uniforms::UniformBuffer::new(
            self.display(),
            Materials {
                ambient: [0.1, 0.1, 0.1, 1.0],
                diffuse: [1.0, 1.0, 1.0, 1.0],
                specular: [1.0, 1.0, 1.0, 1.0],
                shine: 16.0,
            },
        ).unwrap();

        let grid = grid::make_noise_grid(&noise, (50, 50));
        let (vertices, indices) = grid.gen_vertex_buffer();

        let vertex_buffer = glium::VertexBuffer::new(self.display(), &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            self.display(),
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            //polygon_mode: glium::PolygonMode::Line,
            ..Default::default()
        };

        let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, 20.0_f32))
            * Matrix4::from_nonuniform_scale(1.0, 1.0, 25.0)
            * Matrix4::from_scale(1.0);

        println!("{:?}", model);

        let normal_mat = Matrix3::new(
            model[0][0],
            model[0][1],
            model[0][2],
            model[1][0],
            model[1][1],
            model[1][2],
            model[2][0],
            model[2][1],
            1.0,
        );

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.shader_program,
                &uniform! {
                    perspective: cgmath::conv::array4x4(perspective),
                    view: cgmath::conv::array4x4(view),
                    model: cgmath::conv::array4x4(model),
                    normal_model: cgmath::conv::array3x3(normal_mat),
                    Lights: &light_uniforms,
                    Materials: &material_uniforms,
                },
                &draw_params,
            )
            .unwrap();
    }
}
