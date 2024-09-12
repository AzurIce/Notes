use std::{path::Path, sync::Arc};

use glam::Vec3;
use image::{ImageBuffer, Rgb};

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
    pub fn new(
        scale: f32,
        even: Arc<Box<dyn Texture + Send + Sync>>,
        odd: Arc<Box<dyn Texture + Send + Sync>>,
    ) -> Self {
        let inv_scale = 1.0 / scale;
        Self {
            inv_scale,
            even,
            odd,
        }
    }
}

impl Texture for SolidCheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let p = point
            .to_array()
            .map(|v| (self.inv_scale * v).floor() as i32)
            .iter()
            .sum::<i32>();

        if p % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

pub struct CheckerTexture {
    lng_scale: u32, // 经
    lat_scale: u32, // 纬
    even: Arc<Box<dyn Texture + Send + Sync>>,
    odd: Arc<Box<dyn Texture + Send + Sync>>,
}

impl CheckerTexture {
    pub fn new(
        lng_scale: u32,
        lat_scale: u32,
        even: Arc<Box<dyn Texture + Send + Sync>>,
        odd: Arc<Box<dyn Texture + Send + Sync>>,
    ) -> Self {
        Self {
            lng_scale,
            lat_scale,
            even,
            odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let p = [u * self.lng_scale as f32, v * self.lat_scale as f32]
            .map(|v| v.floor() as i32)
            .iter()
            .sum::<i32>();

        if p % 2 == 0 {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

pub struct ImageTexture {
    // ! Use ImageBuffer directly causes rayon error, so use Vec<u8> instead
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self {
        let width = image.width();
        let height = image.height();
        let data = image.to_vec();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let image = image::open(path).unwrap().to_rgb8();
        Self::new(image)
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: Vec3) -> Vec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.width as f32) as usize;
        let j = (v * self.height as f32) as usize;

        let index = (j * self.width as usize + i) * 3;
        let rgb = self.data.get(index..index + 3).unwrap();

        Vec3::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        )
    }
}
