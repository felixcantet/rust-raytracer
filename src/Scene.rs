use crate::Hittable::Hittable;
use glam::Vec3;
use std::any::Any;
use std::borrow::Borrow;
use crate::Ray::{Ray};
use crate::HitInfoCollector::HitInfoCollector;
use crate::Camera::Camera;
use crate::VecExtension::VectorExtension;


pub struct Scene{
    pub objects: Vec<Box<dyn Hittable>>,
    pub camera: Camera
}

impl Scene{

    pub fn add_object_to_scene(&mut self, obj: Box<dyn Hittable>){
        self.objects.push(obj);
    }

    pub fn closest_intersected(&self, ray: &mut Ray) -> HitInfoCollector {
        let mut closest_t = f32::INFINITY;
        let mut closestHitInfo = HitInfoCollector::default();
        for object in self.objects.iter()
        {
            let mut hitInfo = HitInfoCollector::default();
            if object.intersect(&ray, &mut hitInfo){
                if hitInfo.t < closest_t{
                    closest_t = hitInfo.t;
                    closestHitInfo = hitInfo.clone();
                }
            }
        }
        closestHitInfo
    }
}

#[test]
fn test_scene_construction(){
    let mut scene = Scene
    {
        objects: Vec::new(),
        camera: Camera::new(20.0, Vec3::zero(), Vec3::one())
    };
    use crate::Sphere::Sphere;
    let sphere = Sphere::new(Vec3::default(), Vec3::default());
    scene.add_object_to_scene(Box::new(sphere));

    assert_eq!(scene.objects.len(), 1);

    let sphere = Sphere::new(Vec3::default(), Vec3::default());
    scene.add_object_to_scene(Box::new(sphere));
    assert_eq!(scene.objects.len(), 2);
}

#[test]
fn test_intersect_scene(){
    let mut scene = Scene
    {
        objects: Vec::new(),
        camera: Camera::default()
    };
    let zero = Vec3::zero();
    use crate::Sphere::Sphere;
    let sphere = Sphere::new(Vec3::default(), Vec3::new(1.0, 1.0, 1.0));
    scene.add_object_to_scene(Box::new(sphere));

    let ray = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let intersected = scene.closest_intersected(&ray);
    println!("{:?}", intersected);
    assert_eq!(intersected.hitNormal.round(), Vec3::new(-1.0, 0.0, 0.0))
}