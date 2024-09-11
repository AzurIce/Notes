use std::sync::Arc;

use glam::Vec3;
use rand::random;

use crate::{
    texture::Texture, utils::{random_in_unit_sphere, reflectance, refract}, HitRecord, Ray
};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    texture: Arc<Box<dyn Texture + Send + Sync>>,
}

impl Lambertian {
    pub fn new(texture: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = record.normal + random_in_unit_sphere();
        if scatter_direction.length_squared() <= f32::EPSILON {
            scatter_direction = record.normal;
        }

        let scattered_ray = Ray::new(record.point, scatter_direction);
        let attenuation = self.texture.value(record.u, record.v, record.point);
        Some((attenuation, scattered_ray))
    }
}

pub struct Metal {
    texture: Arc<Box<dyn Texture + Send + Sync>>,
    fuzz: f32,
}

impl Metal {
    pub fn new(texture: Arc<Box<dyn Texture + Send + Sync>>) -> Self {
        Metal {
            texture,
            fuzz: 0.0,
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
            let attenuation = self.texture.value(record.u, record.v, record.point);
            Some((attenuation, scattered_ray))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Default for Dielectric {
    fn default() -> Self {
        Dielectric {
            refraction_index: 1.5,
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0); // glass doesn't absorb any light

        let ri = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-ray.direction.normalize()).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scattered = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random() {
            ray.direction.reflect(record.normal)
        } else {
            refract(ray.direction.normalize(), record.normal, ri)
        };

        let scattered_ray = Ray::new(record.point, scattered);

        Some((attenuation, scattered_ray))
    }
}
