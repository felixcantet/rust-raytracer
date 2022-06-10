use crate::Transform::Transform;
use glam::{Vec3, Mat4};
use crate::Ray::Ray;
use crate::VecExtension::VectorExtension;

#[derive(Default)]
pub struct Camera{
    pub transform: Transform,
    focal: f32,
}

impl Camera{
    pub fn new(focal: f32, location: Vec3, target: Vec3) -> Self{
        let mut cam = Camera::default();
        cam.transform.look_at(location, target);
        cam.focal = focal;
        cam
    }

    pub fn get_ray(&self, x: f32, y:f32, aspect_ratio: f32) -> Ray{
        let focalPoint = Vec3::new(0.0, 0.0, self.focal);
        let W = 2.0;
        let H = 2.0;
        let w = 1.0;
        let h = 1.0;

        let screenX = x * (W / w) - (W / 2.0);
        let screenY = y * (H / h) - (H / 2.0);

        let screen_pos = Vec3::new(screenX, screenY / aspect_ratio, 0.0);
        let dir = screen_pos - focalPoint;
        let mut ray = Ray::new(screen_pos, dir.normalize());
        let mut global_ray = self.transform.transform_ray_from_local_to_global(&ray);
        global_ray.normalize();

        global_ray
    }
}

#[test]
fn test_get_ray()
{
    let mut camera = Camera::new(1.0, Vec3::right(), Vec3::zero());
    let ray = camera.get_ray(0.0, 0.0, 16.0 / 9.0);
}