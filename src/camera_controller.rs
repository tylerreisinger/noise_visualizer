use cgmath::{Deg, InnerSpace, Matrix4, PerspectiveFov, Point3, Rad, Vector3};
use glium::glutin::{self, KeyboardInput, MouseScrollDelta};

pub struct CameraController {
    theta: f32,
    phi: f32,
    position: Vector3<f32>,
    up: Vector3<f32>,
    fov: f32,
}

impl CameraController {
    pub fn new(
        position: Vector3<f32>,
        up: Vector3<f32>,
        fov: f32,
        theta: f32,
        phi: f32,
    ) -> CameraController {
        CameraController {
            theta,
            phi,
            position,
            up,
            fov,
        }
    }

    pub fn make_view_perspective_matrix(
        &self,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> (Matrix4<f32>, Matrix4<f32>) {
        let pos = Point3::new(self.position.x, self.position.y, self.position.z);
        let view = Matrix4::look_at_dir(pos, self.facing(), self.up);
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad::from(Deg(self.fov)),
            aspect,
            near,
            far,
        });

        (view, perspective)
    }

    pub fn facing(&self) -> Vector3<f32> {
        let x = self.theta.sin() * self.phi.cos();
        let y = self.theta.cos();
        let z = self.theta.sin() * self.phi.sin();

        Vector3::new(x, y, z)
    }

    pub fn handle_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        const FOV_SPEED: f32 = 0.05;
        const LINE_SIZE: f32 = 10.0;

        match *delta {
            MouseScrollDelta::LineDelta(_, y) => {
                self.fov += y * FOV_SPEED * LINE_SIZE;
            }
            MouseScrollDelta::PixelDelta(_, y) => {
                self.fov += y * FOV_SPEED;
            }
        }

        self.fov = self.fov.min(150.0);
        self.fov = self.fov.max(0.5);
    }

    pub fn handle_mouse_move(&mut self, movement: (f64, f64)) {
        const MOVE_SPEED: f32 = 0.0025;
        let (dx, dy) = (movement.0 as f32, movement.1 as f32);

        self.theta -= MOVE_SPEED * dy;
        self.phi += MOVE_SPEED * dx;
    }

    pub fn handle_keyboard_input(&mut self, input: &KeyboardInput) {
        const MOVE_SPEED: f32 = 2.00;
        const ANGLE_SPEED: f32 = 0.05;
        if input.state == glutin::ElementState::Pressed {
            if let Some(key) = input.virtual_keycode {
                match key {
                    glutin::VirtualKeyCode::A | glutin::VirtualKeyCode::D => {
                        let facing = self.facing();
                        let up = *self.up();
                        let movement = facing.cross(up).normalize();

                        let sign = if key == glutin::VirtualKeyCode::A {
                            1.0
                        } else {
                            -1.0
                        };

                        self.position = self.position() + movement * sign;
                    }
                    glutin::VirtualKeyCode::W | glutin::VirtualKeyCode::S => {
                        let facing = self.facing();
                        let movement = facing * MOVE_SPEED;
                        let sign = if key == glutin::VirtualKeyCode::W {
                            1.0
                        } else {
                            -1.0
                        };
                        self.position = self.position() + movement * sign;
                    }
                    glutin::VirtualKeyCode::Q | glutin::VirtualKeyCode::E => {
                        let up = *self.up();
                        let movement = up * MOVE_SPEED;
                        let sign = if key == glutin::VirtualKeyCode::Q {
                            1.0
                        } else {
                            -1.0
                        };
                        self.position = self.position() + movement * sign;
                    }
                    glutin::VirtualKeyCode::Z | glutin::VirtualKeyCode::X => {
                        let sign = if key == glutin::VirtualKeyCode::Z {
                            1.0
                        } else {
                            -1.0
                        };
                        self.phi -= ANGLE_SPEED * sign;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }
    pub fn phi(&self) -> f32 {
        self.phi
    }
    pub fn fov(&self) -> f32 {
        self.fov
    }
    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }
    pub fn up(&self) -> &Vector3<f32> {
        &self.up
    }
    pub fn position_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.position
    }
    pub fn up_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.up
    }

    pub fn set_theta(&mut self, theta: f32) -> &mut CameraController {
        self.theta = theta;
        return self;
    }
    pub fn set_phi(&mut self, phi: f32) -> &mut CameraController {
        self.phi = phi;
        return self;
    }
    pub fn set_fov(&mut self, fov: f32) -> &mut CameraController {
        self.fov = fov;
        return self;
    }
    pub fn set_position(&mut self, position: Vector3<f32>) -> &mut CameraController {
        self.position = position;
        return self;
    }
    pub fn set_up(&mut self, up: Vector3<f32>) -> &mut CameraController {
        self.up = up;
        return self;
    }
}
