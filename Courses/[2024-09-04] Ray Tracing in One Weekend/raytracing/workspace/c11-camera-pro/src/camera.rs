use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path::Path, time::Instant};

use crate::{log::logger, utils::linear_to_gamma, Hittable, Ray};
use ::log::info;
use glam::Vec3;
use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;
use rand::random;

pub fn ray_color<W: Hittable>(ray: &Ray, world: &W, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::ZERO;
    }

    let unit_direction = ray.direction.normalize();

    // use 0.001 to avoid shadow acne
    if let Some(record) = world.hit(ray, 0.001..f32::INFINITY) {
        return record
            .material
            .scatter(ray, &record)
            .map(|(attenuation, scattered_ray)| {
                attenuation * ray_color(&scattered_ray, world, depth - 1)
            })
            .unwrap_or(Vec3::ZERO);
    }

    let a = 0.5 * (unit_direction.y + 1.0); // 从 [-1, 1] 映射到 [0, 1]
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

pub struct Camera {
    focal_length: f32,
    fov: f32,
    aspect_ratio: f32,

    pos: Vec3,
    look_at: Vec3,
    up: Vec3,

    samples_per_pixel: u32,
    max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let focal_length = 1.0;
        let fov = 90.0f32;

        Self {
            aspect_ratio,
            focal_length,
            fov,

            pos: Vec3::ZERO,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),

            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            aspect_ratio,
            ..Default::default()
        }
    }

    pub fn pos(mut self, pos: Vec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn look_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: Vec3) -> Self {
        self.up = up.normalize();
        self
    }

    pub fn focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    pub fn set_fov(&mut self, fov: f32) -> &mut Self {
        self.fov = fov;
        self
    }

    pub fn set_focal_length(&mut self, focal_length: f32) -> &mut Self {
        self.focal_length = focal_length;
        self
    }

    pub fn set_pos(&mut self, pos: Vec3) -> &mut Self {
        self.pos = pos;
        self
    }

    pub fn set_look_at(&mut self, look_at: Vec3) -> &mut Self {
        self.look_at = look_at;
        self
    }

    pub fn set_up(&mut self, up: Vec3) -> &mut Self {
        self.up = up.normalize();
        self
    }
}

impl Camera {
    pub fn render_to_path<W: Hittable + Send + Sync>(&self, world: &W, output_width: u32, path: impl AsRef<Path>) {
        let back = (self.pos - self.look_at).normalize();
        let right = self.up.cross(back).normalize();
        let up = back.cross(right).normalize();

        let h = self.focal_length * (self.fov / 2.0).to_radians().tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * self.aspect_ratio;
        let viewport_u = viewport_width * right;
        let viewport_v = -viewport_height * up;

        let output_height = (output_width as f32 / self.aspect_ratio) as u32;
        let pixel_delta_u = viewport_u / output_width as f32;
        let pixel_delta_v = viewport_v / output_height as f32;

        let viewport_upper_left = self.pos
            - self.focal_length * back
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let calc_pixel_color = |x: u32, y: u32| {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);

            let color = (0..self.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let rand_offset = Vec3::new(
                        (random::<f32>() - 0.5) * pixel_delta_u.x,
                        (random::<f32>() - 0.5) * pixel_delta_v.y,
                        0.0,
                    );

                    let ray = Ray::new(
                        self.pos,
                        pixel_center + rand_offset - self.pos,
                    );

                    ray_color(&ray, world, self.max_depth)
                })
                .sum::<Vec3>()
                / self.samples_per_pixel as f32;

            let color = linear_to_gamma(color);

            let color_u8 = (255.999 * color).clamp(Vec3::ZERO, Vec3::splat(255.0));
            Rgb([color_u8.x as u8, color_u8.y as u8, color_u8.z as u8])
        };

        let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(output_width, output_height);
        // 防止等半天渲染完了才写入失败，要是失败干脆就先失败，现在不失败后面应该也不失败（有点蠢的方法）
        image.save(&path).unwrap();

        let t = Instant::now();
        info!("generating image...");
        let multi = logger().multi();
        let pb = multi.add(ProgressBar::new((output_height * output_width) as u64));
        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            *pixel = calc_pixel_color(x, y);
            pb.inc(1);
        });
        pb.finish();
        multi.remove(&pb);

        image.save(&path).unwrap();
        info!("cost: {:?}", t.elapsed());
    }
}
