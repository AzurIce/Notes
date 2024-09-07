use glam::Vec3;

use crate::{utils::random_in_unit_sphere, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = record.normal + random_in_unit_sphere();
        if scatter_direction.length_squared() <= f32::EPSILON {
            scatter_direction = record.normal;
        }

        let scattered_ray = Ray::new(record.point, scatter_direction);
        Some((self.albedo, scattered_ray))
    }
}

#[derive(Default)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal {
            albedo,
            ..Default::default()
        }
    }

    pub fn fuzz(mut self, fuzz: f32) -> Self {
        self.fuzz = fuzz;
        self
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray.direction.reflect(record.normal);
        let reflected = reflected.normalize() + self.fuzz * random_in_unit_sphere();

        if reflected.dot(record.normal) > 0.0 {
            let scattered_ray = Ray::new(record.point, reflected);
            Some((self.albedo, scattered_ray))
        } else {
            None
        }
    }
}
