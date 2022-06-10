use glam::Vec3;

pub trait VectorExtension{
    fn zero() -> Self;
    fn one() -> Self;
    fn right() -> Self;
    fn left() -> Self;
    fn up() -> Self;
    fn down() -> Self;
    fn forward() -> Self;
    fn back() -> Self;
}

impl VectorExtension for Vec3{
    fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    fn right() -> Self {
        Vec3::new(1.0, 0.0, 0.0)
    }

    fn left() -> Self {
        Vec3::new(-1.0, 0.0, 0.0)
    }

    fn up() -> Self {
        Vec3::new(0.0, 1.0, 0.0)
    }

    fn down() -> Self {
        Vec3::new(0.0, -1.0, 0.0)
    }

    fn forward() -> Self {
        Vec3::new(0.0, 0.0, 1.0)
    }

    fn back() -> Self {
        Vec3::new(0.0, 0.0, -1.0)
    }
}