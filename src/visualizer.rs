use std::ops::Deref;
use std::f32;
use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::Path;

use glium::uniforms;
use glium::glutin;
use glium::{self, Surface};
use cgmath::{self, InnerSpace, Matrix, Matrix3, Matrix4, SquareMatrix, Vector3, Vector4};
use image;

use camera_controller::CameraController;
use geom;

const MAX_MATERIALS: usize = 5;

#[derive(Copy, Clone)]
struct Lights {
    light_color: [f32; 4],
    light_pos: [f32; 3],
}
implement_uniform_block!(Lights, light_color, light_pos);

#[derive(Copy, Clone, Debug, Default)]
pub struct Material {
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub shine: f32,
    pub _padding: [f32; 3],
}
implement_uniform_block!(Material, ambient, diffuse, specular, shine);

#[derive(Clone, Copy, Debug, Default)]
pub struct Materials {
    pub mat: [Material; MAX_MATERIALS],
}
implement_uniform_block!(Materials, mat);

pub type Index = u32;
pub use grid::Vertex;

pub struct Visualizer {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    running: bool,
    shader_program: glium::Program,
    camera_controller: RefCell<CameraController>,
    geometry: Option<RefCell<Box<geom::GeometryProvider<Vertex, Index>>>>,
    is_wireframe: bool,
    is_focused: bool,
    materials: Option<glium::uniforms::UniformBuffer<Materials>>,

    textures: Vec<glium::texture::Texture2d>,
    update_method: Option<Box<FnMut()>>,
}

impl Visualizer {
    pub fn new(window_builder: glutin::WindowBuilder) -> Visualizer {
        let events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .with_vsync(false)
            .with_gl_profile(glutin::GlProfile::Core);
        let camera_controller = CameraController::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            f32::consts::PI / 2.0,
            f32::consts::PI / 2.0,
        );

        let display = glium::Display::new(window_builder, context, &events_loop).unwrap();

        let shader_program = program!(&display,
            330 => {
                vertex: include_str!("glsl/lighting_per_pixel_vert.glsl"),
                fragment: include_str!("glsl/lighting_per_pixel_frag.glsl"),
        }).unwrap();

        Visualizer {
            events_loop,
            display: display,
            running: true,
            shader_program,
            camera_controller: RefCell::new(camera_controller),
            geometry: None,
            is_wireframe: false,
            is_focused: true,
            materials: None,

            textures: Vec::new(),
            update_method: None,
        }
    }

    pub fn display(&self) -> &glium::Display {
        &self.display
    }
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    pub fn set_geometry(&mut self, geom: Box<geom::GeometryProvider<Vertex, Index>>) {
        self.geometry = Some(RefCell::new(geom));
    }
    pub fn set_materials(&mut self, materials: uniforms::UniformBuffer<Materials>) {
        self.materials = Some(materials);
    }

    pub fn run(&mut self) {
        let mut running = self.running;
        self.load_textures();

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
        let mut is_wireframe = self.is_wireframe;
        let mut is_focused = self.is_focused;

        let size = self.display.gl_window().get_inner_size().unwrap();

        self.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => is_closing = true,
                glutin::WindowEvent::Focused(val) => {
                    is_focused = val;
                }
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::VirtualKeyCode::V => {
                                if input.state == glutin::ElementState::Pressed {
                                    is_wireframe = !is_wireframe;
                                }
                            }
                            _ => (),
                        }
                    }
                    camera_controller.handle_keyboard_input(&input);
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    let (dx, dy) = (
                        position.0 - (size.0 as f64) / 2.0,
                        position.1 - (size.1 as f64) / 2.0,
                    );
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

        self.running = !is_closing;
        self.is_wireframe = is_wireframe;
        self.is_focused = is_focused;

        if is_focused {
            self.display
                .gl_window()
                .deref()
                .set_cursor_position(size.0 as i32 / 2, size.1 as i32 / 2)
                .unwrap();
        }
    }

    fn update(&mut self) {
        if let Some(ref mut f) = self.update_method {
            f();
        }
    }

    pub fn set_update_fn(&mut self, f: Box<FnMut()>) {
        self.update_method = Some(f);
    }

    fn draw(&self, target: &mut glium::Frame) {
        let (mut view, perspective) = self.camera_controller
            .borrow()
            .make_view_perspective_matrix(4.0 / 3.0, 5.0, 1000.0);
        let reflect = build_x_reflection_matrix();
        view = reflect * view;

        let mut geom_provider = self.geometry.as_ref().unwrap().borrow_mut();
        geom_provider.update(self);
        let geom = geom_provider.get_geometry();

        let vertex_buffer = geom.vertex_buffer();
        let index_buffer = geom.index_buffer();
        let model = *geom.model();

        let light_uniforms = glium::uniforms::UniformBuffer::new(
            self.display(),
            Lights {
                light_pos: cgmath::conv::array3(Vector3::new(100.0, -0.0, -500.0_f32).normalize()),
                light_color: cgmath::conv::array4(Vector4::new(1.0, 1.0, 1.0, 1.0)),
            },
        ).unwrap();

        let material_uniforms = self.materials
            .as_ref()
            .expect("Visualizer requires materials to be set");

        let normal_mat = mat4_to_mat3(model).invert().unwrap().transpose();
        let draw_params = self.get_draw_params();

        target
            .draw(
                vertex_buffer,
                index_buffer,
                &self.shader_program,
                &uniform! {
                    perspective: cgmath::conv::array4x4(perspective),
                    view: cgmath::conv::array4x4(view),
                    model: cgmath::conv::array4x4(model),
                    normal_model: cgmath::conv::array3x3(normal_mat),
                    Lights: &light_uniforms,
                    Materials: material_uniforms,
                    grass_texture: self.textures[0].sampled(),
                    dirt_texture: self.textures[1].sampled(),
                    snow_texture: self.textures[2].sampled(),
                    water_texture: self.textures[3].sampled(),
                },
                &draw_params,
            )
            .unwrap();
    }

    pub fn get_draw_params(&self) -> glium::DrawParameters {
        let polygon_mode = if self.is_wireframe {
            glium::PolygonMode::Line
        } else {
            glium::PolygonMode::Fill
        };
        glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            polygon_mode: polygon_mode,
            ..Default::default()
        }
    }

    pub fn load_textures(&mut self) {
        let grass_texture =
            load_texture(self.display(), Path::new("./Assets/grass_texture_1.jpg")).unwrap();
        let dirt_texture = load_texture(
            self.display(),
            Path::new("./Assets/Orange dirtl texture-1.jpg"),
        ).unwrap();
        //let snow_texture = load_texture(self.display(), Path::new("./Assets/snow_texture_1.jpg")).unwrap();
        let stone_texture =
            load_texture(self.display(), Path::new("./Assets/stone_texture_1.jpg")).unwrap();
        let water_texture =
            load_texture(self.display(), Path::new("./Assets/water_texture_1.jpg")).unwrap();

        self.textures.push(grass_texture);
        self.textures.push(dirt_texture);
        self.textures.push(stone_texture);
        self.textures.push(water_texture);
    }
}

fn load_texture(display: &glium::Display, path: &Path) -> io::Result<glium::texture::Texture2d> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let img = image::load(reader, image::ImageFormat::JPEG)
        .unwrap()
        .to_rgba();
    let img_dimensions = img.dimensions();

    let texture_data = glium::texture::RawImage2d::from_raw_rgba(img.into_raw(), img_dimensions);
    Ok(glium::texture::Texture2d::new(display, texture_data).unwrap())
}

fn build_x_reflection_matrix() -> Matrix4<f32> {
    Matrix4::new(
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
    )
}

fn mat4_to_mat3(matrix: Matrix4<f32>) -> Matrix3<f32> {
    Matrix3::new(
        matrix[0][0],
        matrix[0][1],
        matrix[0][2],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
    )
}

impl Materials {
    pub fn new(materials: &[Material]) -> Materials {
        assert!(materials.len() <= MAX_MATERIALS);

        let mut mats = Materials::default();

        for (i, m) in materials.iter().enumerate() {
            mats.mat[i] = *m;
        }

        mats
    }
}
