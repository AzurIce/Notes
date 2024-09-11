use glam::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32) -> Vec3;
}

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32) -> Vec3 {
        self.albedo
    }
}