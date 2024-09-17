pub mod camera;
pub mod log;
pub mod material;
pub mod utils;
pub mod world;
pub mod texture;
pub mod primitive;

use std::{ops::Range, sync::Arc};

use glam::Vec3;
use material::Material;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub point: Vec3,
    /// Unit normal vector
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Arc<Box<dyn Material + Send + Sync>>>,
    pub u: f32,
    pub v: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}