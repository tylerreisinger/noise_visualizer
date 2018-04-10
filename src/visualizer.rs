use std::ops::Deref;
use std::f32;
use std::cell::RefCell;

use glium::glutin;
use glium::{self, Surface};
use cgmath::{self, Matrix4, Vector3};

use camera_controller::CameraController;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
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
                vertex: include_str!("glsl/vertex.glsl"),
                fragment: include_str!("glsl/fragment.glsl"),
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
        let view_projection = reflect
            * self.camera_controller
                .borrow()
                .make_view_perspective_matrix(4.0 / 3.0, 5.0, 100.0);

        let (vertices, indices) = create_cube();

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
            ..Default::default()
        };

        let model =
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 20.0)) * Matrix4::from_scale(10.0);

        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.shader_program,
                &uniform! {
                    view_projection: cgmath::conv::array4x4(view_projection * model)
                },
                &draw_params,
            )
            .unwrap();
    }
}

fn create_cube() -> (Vec<Vertex>, Vec<u32>) {
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
