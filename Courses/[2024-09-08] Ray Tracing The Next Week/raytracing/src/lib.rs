pub mod camera;
pub mod log;
pub mod material;
pub mod utils;
pub mod world;
pub mod texture;

use std::{ops::Range, sync::Arc};

use glam::Vec3;
use material::Material;
use world::bvh::{Aabb, HasAabb};

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

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<Box<dyn Material + Send + Sync>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material + Send + Sync>) -> Self {
        let material = Arc::new(material);
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (-b - sqrtd) / a;
        if t < t_range.start || t_range.end < t {
            t = (-b + sqrtd) / a;
        }
        if t < t_range.start || t_range.end < t {
            return None;
        }

        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: Some(self.material.clone()),
            u: 0.0,
            v: 0.0,
        })
    }
}

impl HasAabb for Sphere {
    fn aabb(&self) -> Aabb {
        Aabb::new(
            self.center - Vec3::splat(self.radius),
            self.center + Vec3::splat(self.radius),
        )
    }
}