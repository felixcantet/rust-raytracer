//mod Transform;

mod Transform;
mod Ray;
mod Camera;
mod HitInfoCollector;
mod Hittable;
mod Sphere;
mod Scene;
mod VecExtension;
mod Renderer;

use glam::{Mat4, Vec4, Affine3A, Vec3, Quat, EulerRot};
use crate::VecExtension::VectorExtension;

fn main() {

    Render();
    return;
    let mut transform = Transform::Transform{
        worldMatrix: Mat4::default(),
        invMatrix: Mat4::default()
    };

    let vec = Vec3::zero();
    let lookAt = Mat4::look_at_lh(Vec3::new(-1.0, 0.0, 0.0), Vec3::new(20.0, 0.0, 5.0), Vec3::new(0.0, 1.0, 0.0));

    let mut camera = Camera::Camera::new(3.0, Vec3::new(-1.0, 0.0, 5.0), Vec3::new(20.0, 0.0, 5.0));
    //camera.transform.worldMatrix = Mat4::default();
    //camera.transform.worldMatrix = camera.transform.worldMatrix.mul_mat4(&lookAt);
    //camera.transform.worldMatrix = Mat4::look_at_lh(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    println!("Cam : {:?}", camera.transform.worldMatrix);

    let (scale, rot, loc) = camera.transform.worldMatrix.to_scale_rotation_translation();
    println!("{}", loc);
    println!("{}", transform.worldMatrix);
    let translation = Vec3::new(10.0, 5.0, 1.0);
    let rotation = Vec3::new(50.0, 50.0, 50.0);
    let scale = Vec3::new(1.0, 2.0, 3.0);

    transform.translate(translation);
    transform.rotate(rotation);
    println!("{:?}", transform.worldMatrix);

}


fn Render()
{
    let mut scene = Scene::Scene{
        objects: Vec::new(),
        camera: Camera::Camera::new(10.0, Vec3::left() * 10.0, Vec3::zero())
    };
    // scene.camera.transform = Transform::Transform::default();
    // scene.camera.transform.translate(Vec3::back() * 10.0);
    // //scene.camera.transform.rotate(Vec3::new(0.0, -180.0, 0.0));
    let sphere = Sphere::Sphere::new(Vec3::zero(), Vec3::one());
    scene.add_object_to_scene(Box::new(sphere));

    // for i in 0..10{
    //     let z = (i - 5) as f32;
    //     let mut sphere = Sphere::Sphere::new(
    //         Vec3::forward() * z,
    //         Vec3::one() * (1.0 / (i + 1) as f32)
    //     );
    //     scene.add_object_to_scene(Box::new(sphere));
    // }

    Renderer::Render(&mut scene, 512, 288);
}

#[test]
fn test_global_to_local(){
    let mut transform = Transform::Transform::default();
    transform.translate(Vec3::right());
    let zero = Vec3::zero();
    let inverse = transform.global_to_local_point(zero);
    assert_eq!(inverse, Vec3::left());

    transform.rotate(Vec3::up() * 90.0_f32.to_radians());
    let forward = Vec3::forward();
    let inverse_dir = transform.global_to_local_vector(forward);
    assert_eq!(inverse_dir.round(), Vec3::left().round());
}

#[test]
fn test_local_to_global(){
    let mut transform = Transform::Transform::default();
    transform.translate(Vec3::right());
    let local_vector = Vec3::zero();
    let global = transform.local_to_global_point(local_vector);
    assert_eq!(global, Vec3::right());
}
#[test]
fn test_look_at_matrix(){
    let mut cam = Camera::Camera::new(10.0, Vec3::right(), Vec3::zero());
    let (scale, rot, pos) = cam.transform.worldMatrix.to_scale_rotation_translation();
    println!("Pos : {:?}", pos);
    let mut rot = rot.to_euler(EulerRot::XYZ);
    let r = Vec3::new(rot.0.to_degrees(), rot.1.to_degrees(), rot.2.to_degrees());
    println!("Rot : {:?}", r);
    assert_eq!(pos, Vec3::right());
}