#[macro_use]
extern crate glium;
extern crate noise_lib;
extern crate cgmath;

mod render;
mod grid;

use glium::Surface;
use cgmath::{Point3, Vector3, Matrix4};

const vertex_shader: &'static str = r#"
    #version 330

    uniform mat4 view_projection = mat4(1.0);

    in vec3 position;
    out vec3 frag_position;

    void main() {
        gl_Position = view_projection * vec4(position, 1.0);
        frag_position = position;
    }
"#;
const fragment_shader: &'static str = r#"
    #version 330

    in vec3 frag_position;
    out vec4 color;

    void main() {
        color = vec4(frag_position.z + 0.2, frag_position.z + 0.2, frag_position.z + 0.2, 1.0);
    }
"#;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, color);


fn main() {
    create_window()
}

fn create_window() {
    use glium::glutin;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Noise Visualizer")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;
    while !closed {

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        draw(&mut target, &display);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

fn draw(target: &mut glium::Frame, display: &glium::Display) {
    let mut mesh_grid = grid::Grid::new(3, 3);
    mesh_grid.as_mut()[2] = 0.5;
    mesh_grid.as_mut()[5] = 0.7;
    mesh_grid.as_mut()[8] = 0.8;

    let (vertex_buffer, index_buffer) = mesh_grid.gen_vertex_buffer();

    let camera = build_camera(display);
    let shader_program = glium::program::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

    let triangle = vec![
        Vertex { position: [0.0, 50.0, 20.0], color: [1.0, 0.0, 1.0, 1.0] }, 
        Vertex { position: [-50.0, -50.0, 20.0], color: [1.0, 1.0, 0.0, 1.0] }, 
        Vertex { position: [50.0, -50.0, 20.0], color: [1.0, 0.0, 0.0, 1.0] },
        Vertex { position: [0.0, 10.0, 15.0], color: [1.0, 0.0, 0.0, 1.0] }, 
        Vertex { position: [-10.0, -10.0, 15.0], color: [1.0, 0.0, 0.0, 1.0] }, 
        Vertex { position: [10.0, -10.0, 25.0], color: [0.0, 0.0, 0.0, 1.0] },
    ];

    let vbo = glium::VertexBuffer::new(display, &vertex_buffer).unwrap();
    let ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_buffer).unwrap();

    let model_transform = Matrix4::from_translation(Vector3::new(0.0, 0.0, 15.0)) * Matrix4::from_scale(10.0);

    //let vertex_buffer = glium::VertexBuffer::new(display, &triangle).unwrap();
    //let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    target.draw(&vbo, &ibo, &shader_program, &uniform! {
            view_projection: *AsRef::<[[f32; 4]; 4]>::as_ref(&(camera * model_transform)),
        },
        &Default::default()
    ).unwrap();


}

fn build_camera(display: &glium::Display) -> Matrix4<f32> {
    let perspective = cgmath::frustum(-50.0, 50.0, -50.0, 50.0, 10.0, 50.0);
    let view = Matrix4::look_at_dir(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 1.0, 0.0));
    perspective * view
}
