use glam::{Mat4, Vec3, EulerRot, Quat};
use crate::Ray::Ray;
use std::ops::Index;

#[derive(Default)]
pub struct Transform {
    pub worldMatrix: Mat4,
    pub invMatrix: Mat4,
}

enum Space {
    World,
    Local,
}

impl Transform {
    pub fn update_inverse_matrix(&mut self){
        self.invMatrix = self.worldMatrix.inverse();
    }

    pub fn translate(&mut self, translation: Vec3)
    {
        // Create Translation Matrix from Translation vector
        let translation_mat = Mat4::from_translation(translation);
        // Translate World Matrix
        self.worldMatrix *= translation_mat;
        // Update the inverse Matrix
        self.update_inverse_matrix();
    }

    pub fn rotate(&mut self, rotation: Vec3)
    {
        let rotation_mat = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
        self.worldMatrix *= rotation_mat;
        self.update_inverse_matrix();
    }

    pub fn scale(&mut self, scale: Vec3){
        let scale_mat = Mat4::from_scale(scale);
        self.worldMatrix *= scale_mat;
        self.update_inverse_matrix();
    }

    pub fn get_position(&self) -> Vec3{
        let (scale, rotation, translation) = self.worldMatrix.to_scale_rotation_translation();
        translation
    }

    pub fn get_rotation(&self) -> Quat{
        let (scale, rotation, translation) = self.worldMatrix.to_scale_rotation_translation();
        rotation
    }

    pub fn get_scale(&self) -> Vec3{
        let (scale, rotation, translation) = self.worldMatrix.to_scale_rotation_translation();
        scale
    }

    pub fn transform_ray_from_local_to_global(&self, ray: &Ray) -> Ray{
        Ray{
            origin: self.worldMatrix.transform_point3(ray.origin),
            direction: self.worldMatrix.transform_vector3(ray.direction)
        }
    }

    pub fn transform_ray_from_global_to_local(&self, ray: &Ray) -> Ray{
        Ray{
            origin: self.invMatrix.transform_point3(ray.origin),
            direction: self.invMatrix.transform_vector3(ray.direction)
        }
    }

    pub fn get_forward(&self) -> Vec3{
        Vec3::new(
            self.worldMatrix.x_axis[2],
            self.worldMatrix.y_axis[2],
            self.worldMatrix.z_axis[2]
        )
    }

    pub fn get_right(&self) -> Vec3{
        Vec3::new(
            self.worldMatrix.x_axis[2],
            self.worldMatrix.x_axis[2],
            self.worldMatrix.x_axis[2]
        )
    }

    pub fn get_up(&self) -> Vec3{
        Vec3::new(
            self.worldMatrix.y_axis[0],
            self.worldMatrix.y_axis[1],
            self.worldMatrix.y_axis[2]
        )
    }

    pub fn look_at(&mut self, pos: Vec3, target: Vec3){
        let up = Vec3::new(0.0, 1.0, 0.);
        let pos = pos;
        let target_pos = target;

        let forward = (target_pos - pos).normalize();
        let right = up.cross(forward).normalize();

        let up = forward.cross(right).normalize();

        let x = right.dot(pos);
        let y = up.dot(pos);
        let z = forward.dot(pos);

        // X Axis
        self.worldMatrix.x_axis.x = right.x;
        self.worldMatrix.x_axis.y = up.x;
        self.worldMatrix.x_axis.z = forward.x;

        self.worldMatrix.x_axis.w = 0.0;

        // Y Axis
        self.worldMatrix.y_axis.x = right.y;
        self.worldMatrix.y_axis.y = up.y;
        self.worldMatrix.y_axis.z = forward.y;

        self.worldMatrix.y_axis.w = 0.0;

        // Z Axis
        self.worldMatrix.z_axis.x = right.z;
        self.worldMatrix.z_axis.y = up.z;
        self.worldMatrix.z_axis.z = forward.z;

        self.worldMatrix.z_axis.w = 0.0;

        // W Axis
        self.worldMatrix.w_axis.x = pos.x;
        self.worldMatrix.w_axis.y = pos.y;
        self.worldMatrix.w_axis.z = pos.z;
        self.worldMatrix.w_axis.w = 1.0;

        self.update_inverse_matrix();
    }

    pub fn local_to_global_point(&self, point: Vec3) -> Vec3{
        self.worldMatrix.transform_point3(point)
    }

    pub fn global_to_local_point(&self, point: Vec3) -> Vec3{
        self.invMatrix.transform_point3(point)
    }

    pub fn local_to_global_vector(&self, vector: Vec3) -> Vec3{
        self.worldMatrix.transform_vector3(vector)
    }

    pub fn global_to_local_vector(&self, vector: Vec3) -> Vec3{
        self.invMatrix.transform_vector3(vector)
    }
}

