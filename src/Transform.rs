use glam::{Mat4, Vec3, EulerRot, Quat};

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
}