use std::ops::Range;

use glam::Vec3;

use crate::{HitRecord, Hittable, Ray};

pub trait HasAabb {
    fn aabb(&self) -> Aabb;
}

pub trait AabbHittable: Hittable + HasAabb {}

impl<T: HasAabb + Hittable> AabbHittable for T {}

#[derive(Debug, Clone)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, mut max: Vec3) -> Self {
        const DELTA: f32 = 0.0001;

        if max.x - min.x < DELTA {
            max.x += DELTA;
        }
        if max.y - min.y < DELTA {
            max.y += DELTA;
        }
        if max.z - min.z < DELTA {
            max.z += DELTA;
        }

        Aabb { min, max }
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        Aabb::new(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn longest_axis(&self) -> usize {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        let z = self.max.z - self.min.z;
        let max = x.max(y).max(z);

        let arr = [x, y, z];
        arr.iter().position(|&x| x == max).unwrap()
    }
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut t_min = t_range.start;
        let mut t_max = t_range.end;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let t1 = (self.max[i] - ray.origin[i]) * inv_d;

            t_min = t_min.max(t0.min(t1));
            t_max = t_max.min(t0.max(t1));

            // println!("t_min: {}, t_max: {}", t_min, t_max);

            if t_max <= t_min {
                return None;
            }
        }

        let point = ray.at(t_min);
        Some(HitRecord {
            point,
            normal: Vec3::ZERO,
            t: t_min,
            front_face: false,
            material: None,
            u: 0.0,
            v: 0.0,
        })
    }
}

pub enum BvhNode {
    Leaf(Box<dyn AabbHittable + Send + Sync>),
    Node {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        aabb: Aabb,
    },
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        match self {
            BvhNode::Leaf(object) => object.hit(ray, t_range),
            BvhNode::Node { left, right, aabb } => {
                if aabb.hit(ray, t_range.clone()).is_none() {
                    return None;
                }
                let hit_left = left.hit(ray, t_range.clone());
                let hit_right = right.hit(
                    ray,
                    t_range.start..hit_left.as_ref().map(|rec| rec.t).unwrap_or(t_range.end),
                );
                hit_right.or(hit_left)
            }
        }
    }
}

impl HasAabb for BvhNode {
    fn aabb(&self) -> Aabb {
        match self {
            BvhNode::Leaf(object) => object.aabb(),
            BvhNode::Node { aabb, .. } => aabb.clone(),
        }
    }
}

impl BvhNode {
    pub fn from_objects(mut objects: Vec<Box<dyn AabbHittable + Send + Sync>>) -> Self {
        let aabb = objects
            .iter()
            .map(|obj| obj.aabb())
            .reduce(|a, b| a.union(&b))
            .unwrap();
        let axis = aabb.longest_axis();
        objects.sort_by(|a, b| a.aabb().min[axis].partial_cmp(&b.aabb().min[axis]).unwrap());

        return if objects.len() == 1 {
            let object = objects.remove(0);
            BvhNode::Leaf(object)
        } else {
            let left = Box::new(BvhNode::from_objects(
                objects.drain(..objects.len() / 2).collect(),
            ));
            let right = Box::new(BvhNode::from_objects(objects));
            // let aabb = left.aabb().union(&right.aabb());
            BvhNode::Node { left, right, aabb }
        };
    }
}
