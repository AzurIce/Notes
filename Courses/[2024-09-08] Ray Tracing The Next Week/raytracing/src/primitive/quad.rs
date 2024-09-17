use std::sync::Arc;

use glam::Vec3;

use crate::{
    material::Material,
    world::bvh::{Aabb, HasAabb},
    HitRecord, Hittable,
};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<Box<dyn Material + Send + Sync>>,
    
    /// followings are cached values
    normal: Vec3, // normalized
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<Box<dyn Material + Send + Sync>>) -> Self {
        let n = u.cross(v);
        
        let w = n / n.dot(n);
        let normal = n.normalize();
        Quad { q, u, v, material, normal, w }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::Ray, t_range: std::ops::Range<f32>) -> Option<crate::HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < f32::EPSILON {
            return None;
        }

        let d = self.normal.dot(self.q);
        let t = (d - self.normal.dot(ray.origin)) / denom;
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let point = ray.at(t);
        let point_on_plane = point - self.q;
        let u = self.w.dot(point_on_plane.cross(self.v));
        let v = self.w.dot(self.u.cross(point_on_plane));
        if !(0.0 <= u && u <= 1.0 && 0.0 <= v && v <= 1.0) {
            return None;
        }

        let front_face = self.normal.dot(ray.direction) > 0.0;

        Some(HitRecord {
            t,
            point,
            normal: self.normal,
            front_face,
            u,
            v,
            material: Some(self.material.clone()),
        })
    }
}

impl HasAabb for Quad {
    fn aabb(&self) -> Aabb {
        Aabb::new(self.q, self.q + self.u + self.v)
            .union(&Aabb::new(self.q + self.u, self.q + self.v))
    }
}

#[cfg(test)]
mod test {
    use crate::{material::Lambertian, texture::SolidColor, Ray};

    use super::*;

    #[test]
    fn test_quad_hit() {
        let quad = Quad::new(
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::X * 2.0,
            Vec3::Y * 2.0,
            Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
                SolidColor::new(Vec3::new(1.0, 0.0, 0.0)),
            ))))),
        );

        println!("{:?}", quad.aabb());
        let ray = Ray::new(Vec3::ZERO, Vec3::Z);
        let hit = quad.aabb().hit(&ray, 0.0..f32::INFINITY);
        assert!(hit.is_none());

        let ray = Ray::new(Vec3::ZERO, Vec3::Z);
        let hit = quad.hit(&ray, 0.0..1.0);
        assert!(hit.is_some());
        let ray = Ray::new(Vec3::ZERO, Vec3::new(2.0, 0.0, 1.0));
        let hit = quad.hit(&ray, 0.0..1.0);
        assert!(hit.is_some());
        let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 2.0, 1.0));
        let hit = quad.hit(&ray, 0.0..1.0);
        assert!(hit.is_some());
        let ray = Ray::new(Vec3::ZERO, Vec3::new(2.0, 2.0, 1.0));
        let hit = quad.hit(&ray, 0.0..1.0);
        assert!(hit.is_some());
    }
}
