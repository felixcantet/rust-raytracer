use crate::Ray::Ray;
use glam::{Vec3, Vec2};
use crate::HitInfoCollector::HitInfoCollector;

pub trait Hittable : Sync + Send{
    fn intersect(&self, ray: &Ray, hit: &mut HitInfoCollector) -> bool;
    // fn intersect(&self, ray: &Ray, impact: &mut Vec3) -> bool;
    // fn get_normal(&self, obs: &Vec3, impact: &Vec3) -> Vec3;
    // fn get_uv(&self, impact: &Vec3) -> Vec2;
}