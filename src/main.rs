//mod Transform;

mod Transform;

use glam::{Mat4, Vec4, Affine3A, Vec3};

fn main() {
    let mut transform = Transform::Transform{
        worldMatrix: Mat4::default(),
        invMatrix: Mat4::default()
    };

    let translation = Vec3::new(10.0, 5.0, 1.0);
    let rotation = Vec3::new(50.0, 50.0, 50.0);
    let scale = Vec3::new(1.0, 2.0, 3.0);

    transform.translate(translation);
    transform.rotate(rotation);
    println!("{:?}", transform.worldMatrix);

}
