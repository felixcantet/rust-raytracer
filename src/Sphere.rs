use crate::Transform::Transform;
use crate::Hittable::Hittable;
use crate::Ray::Ray;
use crate::HitInfoCollector::HitInfoCollector;
use glam::{Vec3, Vec2};
use crate::Camera::Camera;

pub struct Sphere{
    pub transform: Transform
}

impl Sphere{

    pub fn new(location: Vec3, scale: Vec3) -> Self{
        let mut transform = Transform::default();
        transform.translate(location);
        transform.scale(scale);
        Sphere{
            transform
        }
    }

    pub fn get_normal(&self, obs: &Vec3, impact: &Vec3) -> Ray{
        let impact_local = self.transform.global_to_local_point(*impact);
        let obs_local = self.transform.global_to_local_point(*obs);

        let dotP = impact_local.dot(obs_local);
        let mut sens = 1.0;

        if dotP < 0.0{
            sens = -1.0;
        }

        let dir = impact_local.clone();
        let local_ray = Ray::new(impact_local, dir * sens);
        let mut normal_ray = self.transform.transform_ray_from_local_to_global(&local_ray);
        normal_ray.normalize();
        normal_ray
    }

    pub fn get_uv(&self, impact: &Vec3) -> Vec2{
        let local_impact = self.transform.global_to_local_point(*impact);
        let n = local_impact.clone();

        let pi = std::f32::consts::PI;

        let mut phi = n.z.acos() / pi;
        let mut theta = (pi - n.y.atan2(n.x)) / (2.0 * pi);

        if phi < 0.0{
            phi = 0.0;
        }
        if theta < 0.0{
            theta = 0.0;
        }

        Vec2::new(theta, phi)
    }
}

impl Hittable for Sphere{
    fn intersect(&self, ray: &Ray, hit: &mut HitInfoCollector) -> bool {
        let local_ray = self.transform.transform_ray_from_global_to_local(ray);
        let sphere_to_ray = local_ray.origin - self.transform.get_position();
        let a = local_ray.direction.dot(local_ray.direction);
        //let a = local_ray.direction.length_squared();

        //let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let b = 2.0 * local_ray.direction.dot(local_ray.origin);

        //let b = 2.0 * (local_ray.direction.x * local_ray.origin.x + local_ray.direction.y * local_ray.origin.y + local_ray.direction.z * local_ray.origin.z);

        //let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let c = local_ray.origin.dot(local_ray.origin) - 1.0;
        //let c = (local_ray.origin.x * local_ray.origin.x + local_ray.origin.y * local_ray.origin.y + local_ray.origin.z * local_ray.origin.z) - 1.0;

        let bb = b * b;
        let ac4 = 4.0 * a * c;
        let delta = bb - ac4;

        if delta < 0.0{
            return false;
        }
        let t1 = (-b + delta.sqrt()) / (2.0 * a);
        let t2 = (-b - delta.sqrt()) / (2.0 * a);
        let mut t = 0.0;

        if t1 < t2 && t1 > 0.0
        {
            t = t1
        }
        else if t1 > 0.0 && t2 < 0.0{
            t = t1;
        }
        else{
            t = t2;
        }

        hit.t = t;
        let local_impact = local_ray.at(t);
        let impact_global = self.transform.local_to_global_point(local_impact);
        let normal = self.get_normal(&ray.origin, &impact_global);
        let uv = self.get_uv(&impact_global);

        hit.hitNormal = normal.direction;
        hit.hitPosition = impact_global;
        hit.uv = uv;
        true
    }
}

#[test]
fn test_intersect(){
    let cam = Camera::new(10.0, Vec3::new(-10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

    let ray = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));

    let mut hitInfo = HitInfoCollector::default();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 3.0, 3.0));
    let hit = sphere.intersect(&ray, &mut hitInfo);
    println!("{:?}", hitInfo.t);
    assert_eq!(hitInfo.hitNormal.round(), Vec3::new(-1.0, 0.0, 0.0).round());
    assert_eq!(hitInfo.hitPosition.round(), Vec3::new(-3.0, 0.0, 0.0).round());
}

#[test]
fn test_sphere_builder(){
    let pos = Vec3::new(10.0, 0.0, 0.0);
    let scale = Vec3::new(4.0, 2.0, 2.0);
    let sphere = Sphere::new(pos, scale);
    assert_eq!(sphere.transform.get_position(), pos);
    assert_eq!(sphere.transform.get_scale(), scale);
}