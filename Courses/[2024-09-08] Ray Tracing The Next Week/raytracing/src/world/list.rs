use std::ops::Range;

use crate::{Hittable, HitRecord, Ray};

pub struct List(pub(super) Vec<Box<dyn Hittable + Send + Sync>>);

impl List {
    pub fn from_objects(objects: Vec<Box<dyn Hittable + Send + Sync>>) -> Self {
        List(objects)
    }
}

impl Hittable for List {
    fn hit(&self, ray: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut closest = t_range.end;
        let mut hit_record = None;
        for object in self.0.iter() {
            if let Some(record) = object.hit(ray, t_range.start..closest) {
                closest = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}