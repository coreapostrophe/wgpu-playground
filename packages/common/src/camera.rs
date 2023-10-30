use std::f32::consts::PI;

use cgmath::{Deg, InnerSpace, Matrix4, Point3, Rad, Vector2, Vector3};

pub struct Camera {
    position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new(position: [f32; 3], yaw: Deg<f32>, pitch: Deg<f32>) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
    pub fn position(&self) -> &Point3<f32> {
        &self.position
    }
    pub fn yaw(&self) -> &Rad<f32> {
        &self.yaw
    }
    pub fn pitch(&self) -> &Rad<f32> {
        &self.pitch
    }
    pub fn mut_position(&mut self) -> &mut Point3<f32> {
        &mut self.position
    }
    pub fn mut_yaw(&mut self) -> &mut Rad<f32> {
        &mut self.yaw
    }
    pub fn mut_pitch(&mut self) -> &mut Rad<f32> {
        &mut self.pitch
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                self.pitch.0.cos() * self.yaw.0.cos(),
                self.pitch.0.sin(),
                self.pitch.0.cos() * self.yaw.0.sin(),
            )
            .normalize(),
            Vector3::unit_y(),
        )
    }
}

pub struct CameraController {
    rotation: Vector2<f32>,
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            rotation: Vector2::new(0.0, 0.0),
            speed,
        }
    }

    pub fn set_rotation(&mut self, rotate_x: f32, rotate_y: f32) {
        self.rotation.x = rotate_x;
        self.rotation.y = rotate_y;
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        {
            let camera_yaw = camera.mut_yaw();
            *camera_yaw += Rad(self.rotation.x) * self.speed;
        }

        let camera_pitch = camera.mut_pitch();
        *camera_pitch += Rad(self.rotation.y) * self.speed;

        self.rotation = Vector2::new(0.0, 0.0);

        if *camera_pitch < -Rad(89.0 * PI / 180.0) {
            *camera_pitch = -Rad(89.0 * PI / 180.0);
        } else if *camera_pitch > Rad(89.0 * PI / 180.0) {
            *camera_pitch = Rad(89.0 * PI / 180.0);
        }
    }
}
