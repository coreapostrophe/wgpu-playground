use cgmath::{Matrix4, Point3, Vector3};

use crate::transform::{create_projection, create_transforms, create_view, create_view_projection};

pub struct Projection {
    model_translation: Vector3<f32>,
    model_rotation: Vector3<f32>,
    model_scale: Vector3<f32>,
    model_matrix: Matrix4<f32>,

    camera_position: Point3<f32>,
    look_direction: Point3<f32>,
    up_direction: Vector3<f32>,
    view_matrix: Matrix4<f32>,

    aspect_ratio: f32,
    is_perspective: bool,
    projection_matrix: Matrix4<f32>,

    mvp_matrix: Matrix4<f32>,
}

impl Projection {
    pub fn new(width: f32, height: f32) -> Self {
        let model_translation: Vector3<f32> = [0.0, 0.0, 0.0].into();
        let model_rotation: Vector3<f32> = [0.0, 0.0, 0.0].into();
        let model_scale: Vector3<f32> = [1.0, 1.0, 1.0].into();

        let model_matrix = create_transforms(
            [
                model_translation.x,
                model_translation.y,
                model_translation.z,
            ],
            [model_rotation.x, model_rotation.y, model_rotation.z],
            [model_scale.x, model_scale.y, model_scale.z],
        );

        let camera_position: Point3<f32> = [3.0, 1.5, 3.0].into();
        let look_direction: Point3<f32> = [0.0, 0.0, 0.0].into();
        let up_direction: Vector3<f32> = Vector3::unit_y();
        let aspect_ratio: f32 = width / height;
        let is_perspective: bool = true;

        let (view_matrix, projection_matrix, view_projection_matrix) = create_view_projection(
            [camera_position.x, camera_position.y, camera_position.z].into(),
            [look_direction.x, look_direction.y, look_direction.z].into(),
            up_direction,
            aspect_ratio,
            is_perspective,
        );
        let mvp_matrix = view_projection_matrix * model_matrix;

        Self {
            aspect_ratio,
            camera_position,
            is_perspective,
            look_direction,
            model_rotation,
            model_scale,
            model_translation,
            up_direction,
            model_matrix,
            view_matrix,
            projection_matrix,
            mvp_matrix,
        }
    }

    pub fn model_translation(&self) -> &Vector3<f32> {
        &self.model_translation
    }
    pub fn model_rotation(&self) -> &Vector3<f32> {
        &self.model_rotation
    }
    pub fn model_scale(&self) -> &Vector3<f32> {
        &self.model_scale
    }
    pub fn model_matrix(&self) -> &Matrix4<f32> {
        &self.model_matrix
    }

    pub fn camera_position(&self) -> &Point3<f32> {
        &self.camera_position
    }
    pub fn look_direction(&self) -> &Point3<f32> {
        &self.look_direction
    }
    pub fn up_direction(&self) -> &Vector3<f32> {
        &self.up_direction
    }
    pub fn view_matrix(&self) -> &Matrix4<f32> {
        &self.view_matrix
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn is_perspective(&self) -> bool {
        self.is_perspective
    }
    pub fn projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn mvp_matrix(&self) -> &Matrix4<f32> {
        &self.mvp_matrix
    }

    pub fn model_matrix_slice(&self) -> &[f32; 16] {
        self.model_matrix.as_ref() as &[f32; 16]
    }
    pub fn view_matrix_slice(&self) -> &[f32; 16] {
        self.view_matrix.as_ref() as &[f32; 16]
    }
    pub fn projection_matrix_slice(&self) -> &[f32; 16] {
        self.projection_matrix.as_ref() as &[f32; 16]
    }
    pub fn mvp_matrix_slice(&self) -> &[f32; 16] {
        self.mvp_matrix.as_ref() as &[f32; 16]
    }

    pub fn set_model_translation(&mut self, translation_vector: [f32; 3]) {
        self.model_translation = translation_vector.into();
        self.update_model_matrix();
    }
    pub fn set_model_rotation(&mut self, translation_vector: [f32; 3]) {
        self.model_rotation = translation_vector.into();
        self.update_model_matrix();
    }
    pub fn set_model_scale(&mut self, translation_vector: [f32; 3]) {
        self.model_scale = translation_vector.into();
        self.update_model_matrix();
    }
    pub fn set_model_matrix(
        &mut self,
        translation: [f32; 3],
        rotation: [f32; 3],
        scaling: [f32; 3],
    ) {
        self.model_matrix = create_transforms(translation, rotation, scaling);
        self.update_mvp_matrix();
    }

    pub fn set_camera_position(&mut self, position_point: [f32; 3]) {
        self.camera_position = position_point.into();
        self.update_view_matrix();
    }
    pub fn set_look_direction(&mut self, direction_point: [f32; 3]) {
        self.look_direction = direction_point.into();
        self.update_view_matrix();
    }
    pub fn set_up_direction(&mut self, direction_vector: [f32; 3]) {
        self.up_direction = direction_vector.into();
        self.update_view_matrix();
    }
    pub fn set_view_matrix(
        &mut self,
        camera_position: Point3<f32>,
        look_direction: Point3<f32>,
        up_direction: Vector3<f32>,
    ) {
        self.view_matrix = create_view(camera_position, look_direction, up_direction);
        self.update_mvp_matrix();
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.update_projection_matrix();
    }
    pub fn set_is_perspective(&mut self, is_perspective: bool) {
        self.is_perspective = is_perspective;
        self.update_projection_matrix();
    }
    pub fn set_projection_matrix(&mut self, aspect_ratio: f32, is_perspective: bool) {
        self.projection_matrix = create_projection(aspect_ratio, is_perspective);
        self.update_mvp_matrix();
    }

    pub fn replace_projection_matrix(&mut self, projection_matrix: Matrix4<f32>) {
        self.projection_matrix = projection_matrix;
        self.update_mvp_matrix();
    }
    pub fn replace_model_matrix(&mut self, model_matrix: Matrix4<f32>) {
        self.model_matrix = model_matrix;
        self.update_mvp_matrix();
    }
    pub fn replace_view_matrix(&mut self, view_matrix: Matrix4<f32>) {
        self.view_matrix = view_matrix;
        self.update_mvp_matrix();
    }

    fn update_mvp_matrix(&mut self) {
        self.mvp_matrix = self.projection_matrix * self.view_matrix * self.model_matrix;
    }
    fn update_model_matrix(&mut self) {
        self.set_model_matrix(
            [
                self.model_translation.x,
                self.model_translation.y,
                self.model_translation.z,
            ],
            [
                self.model_rotation.x,
                self.model_rotation.y,
                self.model_rotation.z,
            ],
            [self.model_scale.x, self.model_scale.y, self.model_scale.z],
        )
    }
    fn update_view_matrix(&mut self) {
        self.set_view_matrix(self.camera_position, self.look_direction, self.up_direction);
    }
    fn update_projection_matrix(&mut self) {
        self.set_projection_matrix(self.aspect_ratio, self.is_perspective);
    }
}
