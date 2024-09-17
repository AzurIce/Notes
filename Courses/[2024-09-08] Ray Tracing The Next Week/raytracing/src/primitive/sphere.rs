use std::{f32::consts::PI, ops::Range, sync::Arc};
use glam::Vec3;

use crate::material::Material;
use crate::world::bvh::{Aabb, HasAabb};
use crate::Ray;

use crate::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<Box<dyn Material + Send + Sync>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Box<dyn Material + Send + Sync>>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(&self, point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = point.z.atan2(-point.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
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

        let (u, v) = self.get_uv(normal);

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            material: Some(self.material.clone()),
            u,
            v,
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