use glam::Vec3;

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
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        if t - t_min < f32::EPSILON || t_max - t < f32::EPSILON {
            t = (-b + sqrtd) / a;
        }
        if t - t_min < f32::EPSILON || t_max - t < f32::EPSILON {
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
        })
    }
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut hit_record = None;
        for object in self.iter() {
            if let Some(record) = object.hit(ray, t_min, closest) {
                closest = record.t;
                hit_record = Some(record);
            }
        }
        hit_record
    }
}
