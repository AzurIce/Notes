use std::sync::Arc;

use glam::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
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
    fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct SolidCheckerTexture {
    inv_scale: f32,
    even: Arc<Box<dyn Texture + Send + Sync>>,
    odd: Arc<Box<dyn Texture + Send + Sync>>,
}

impl SolidCheckerTexture {
    pub fn new(scale: f32, even: Arc<Box<dyn Texture + Send + Sync>>, odd: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
        let inv_scale = 1.0 / scale;
        Self { inv_scale, even, odd }
    }
}

impl Texture for SolidCheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let p = point.to_array().map(|v| (self.inv_scale * v).floor() as i32).iter().sum::<i32>();

        if p % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
