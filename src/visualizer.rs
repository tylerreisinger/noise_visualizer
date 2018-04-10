use glium::glutin;
use glium::{self, Surface};
use cgmath::{self, Deg, InnerSpace, Matrix4, Point3, Rad, Vector2, Vector3};

use camera;

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
    camera: camera::Camera,
    shader_program: glium::Program,
}

impl Visualizer {
    pub fn new(window_builder: glutin::WindowBuilder) -> Visualizer {
        let mut events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new();
        let camera = camera::Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 20.0),
            Vector3::new(0.0, 1.0, 0.0),
            cgmath::PerspectiveFov {
                fovy: Rad::from(Deg(45.0)),
                aspect: 4.0 / 3.0,
                near: 4.0,
                far: 100.0f32,
            }.to_perspective(),
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
            camera: camera,
            shader_program,
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
        let mut camera = self.camera.clone();
        self.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => is_closing = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    handle_key_event(&input, &mut camera);
                }
                _ => (),
            },
            _ => (),
        });

        self.running = !is_closing;
        self.camera = camera;
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
        let view_projection = reflect * Matrix4::from(self.camera.clone());

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

fn handle_key_event(input: &glutin::KeyboardInput, camera: &mut camera::Camera) {
    const MOVE_SPEED: f32 = 2.00;
    if input.state == glutin::ElementState::Pressed {
        if let Some(key) = input.virtual_keycode {
            match key {
                glutin::VirtualKeyCode::A | glutin::VirtualKeyCode::D => {
                    let facing = camera.look_distance().normalize();
                    let up = *camera.up();
                    let movement = facing.cross(up).normalize();

                    let sign = if key == glutin::VirtualKeyCode::A {
                        1.0
                    } else {
                        -1.0
                    };

                    let new_look_at = camera.look_at() + movement * sign;
                    let new_position = camera.position() + movement * sign;

                    camera.set_look_at(new_look_at);
                    camera.set_position(new_position);
                }
                glutin::VirtualKeyCode::W | glutin::VirtualKeyCode::S => {
                    let facing = camera.look_distance().normalize();
                    let movement = facing * MOVE_SPEED;
                    let sign = if key == glutin::VirtualKeyCode::W {
                        1.0
                    } else {
                        -1.0
                    };
                    let new_look_at = camera.look_at() + movement * sign;
                    let new_position = camera.position() + movement * sign;

                    camera.set_look_at(new_look_at);
                    camera.set_position(new_position);
                }
                glutin::VirtualKeyCode::Z | glutin::VirtualKeyCode::X => {
                    let up = *camera.up();
                    let movement = up * MOVE_SPEED;
                    let sign = if key == glutin::VirtualKeyCode::Z {
                        1.0
                    } else {
                        -1.0
                    };
                    let new_look_at = camera.look_at() + movement * sign;
                    let new_position = camera.position() + movement * sign;

                    camera.set_look_at(new_look_at);
                    camera.set_position(new_position);
                }
                glutin::VirtualKeyCode::Q | glutin::VirtualKeyCode::E => {
                    let distance = camera.look_distance();

                    let plane_vec = Vector2::new(distance.x, distance.z);
                    let mag = plane_vec.magnitude();
                    let mut theta = f32::atan2(plane_vec.y, plane_vec.x);

                    if key == glutin::VirtualKeyCode::Q {
                        theta += 0.05;
                    } else {
                        theta -= 0.05;
                    }

                    let (sin, cos) = theta.sin_cos();
                    let (x, y) = (mag * cos, mag * sin);

                    let position = *camera.position();

                    let new_look_at =
                        Point3::new(x + position.x, camera.look_at().y, y + position.z);
                    println!("{}, {}, {:?}, {:?}", theta, mag, distance, new_look_at);
                    camera.set_look_at(new_look_at);
                }
                _ => (),
            }
        }
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
