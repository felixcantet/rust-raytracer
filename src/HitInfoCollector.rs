use glam::{Vec3, Vec2};

#[derive(Debug, Copy, Clone)]
pub struct HitInfoCollector{
    pub hitPosition: Vec3,
    pub hitNormal: Vec3,
    pub t: f32,
    pub uv: Vec2,
    //todo: material

}

impl Default for HitInfoCollector
{
    fn default() -> Self {
        HitInfoCollector{
            hitPosition: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            hitNormal: Vec3::new(0.0, 0.0, 0.0),
            t: -1.0,
            uv: Vec2::default()
        }
    }
}