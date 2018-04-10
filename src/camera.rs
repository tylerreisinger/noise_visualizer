use cgmath::{self, Matrix4, Perspective, Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Camera {
    position: Point3<f32>,
    look_at: Point3<f32>,
    up: Vector3<f32>,

    perspective: Perspective<f32>,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        look_at: Point3<f32>,
        up: Vector3<f32>,
        perspective: Perspective<f32>,
    ) -> Camera {
        Camera {
            position,
            look_at,
            up,
            perspective,
        }
    }

    pub fn position(&self) -> &Point3<f32> {
        &self.position
    }
    pub fn position_mut(&mut self) -> &mut Point3<f32> {
        &mut self.position
    }
    pub fn set_position(&mut self, position: Point3<f32>) {
        self.position = position;
    }
    pub fn look_at(&self) -> &Point3<f32> {
        &self.look_at
    }
    pub fn look_at_mut(&mut self) -> &mut Point3<f32> {
        &mut self.look_at
    }
    pub fn set_look_at(&mut self, look_at: Point3<f32>) {
        self.look_at = look_at;
    }
    pub fn up(&self) -> &Vector3<f32> {
        &self.up
    }
    pub fn look_distance(&self) -> Vector3<f32> {
        self.look_at - self.position
    }

    pub fn make_perspective_view_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at(self.position, self.look_at, self.up);
        let perspective = Matrix4::from(self.perspective);

        perspective * view
    }
}

impl From<Camera> for Matrix4<f32> {
    fn from(val: Camera) -> Matrix4<f32> {
        val.make_perspective_view_matrix()
    }
}
