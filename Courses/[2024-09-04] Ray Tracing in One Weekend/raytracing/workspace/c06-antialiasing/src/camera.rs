use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{log::logger, Hittable, Ray, World};
use ::log::info;
use glam::Vec3;
use indicatif::ProgressBar;
use rand::random;

pub fn ray_color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.0..f32::INFINITY) {
        let n = record.normal.normalize();
        return 0.5 * (Vec3::new(n.x, n.y, n.z) + 1.0);
    }

    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0); // 从 [-1, 1] 映射到 [0, 1]
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

pub struct Camera {
    focal_length: f32,
    aspect_ratio: f32,
    viewport_height: f32,
    viewport_width: f32,
    camera_center: Vec3,
    samples_per_pixel: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Self {
            focal_length: 1.0,
            aspect_ratio,
            viewport_height,
            viewport_width,
            camera_center: Vec3::ZERO,
            samples_per_pixel: 100,
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            ..Default::default()
        }
    }

    pub fn camera_center(mut self, camera_center: Vec3) -> Self {
        self.camera_center = camera_center;
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
}

impl Camera {
    pub fn render_to_ppm(
        &self,
        world: &World,
        output_width: u32,
        path: impl AsRef<Path>,
    ) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);

        // Camera
        let viewport_u = Vec3::new(self.viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        let output_height = (output_width as f32 / self.aspect_ratio) as u32;

        let pixel_delta_u = viewport_u / output_width as f32;
        let pixel_delta_v = viewport_v / output_height as f32;

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        info!("generating image...");
        writer
            .write_all(format!("P3\n{} {}\n255", output_width, output_height).as_bytes())
            .unwrap();

        let multi = logger().multi();
        let pb = multi.add(ProgressBar::new((output_height * output_width) as u64));
        for j in 0..output_height {
            for i in 0..output_width {
                let mut color = Vec3::ZERO;

                let pixel_center =
                    pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
                for _ in 0..self.samples_per_pixel {
                    let rand_offset = Vec3::new(
                        (random::<f32>() - 0.5) * pixel_delta_u.x,
                        (random::<f32>() - 0.5) * pixel_delta_v.y,
                        0.0,
                    );

                    let ray = Ray::new(
                        self.camera_center,
                        pixel_center + rand_offset - self.camera_center,
                    );

                    color += ray_color(&ray, world);
                }
                color /= self.samples_per_pixel as f32;

                let color_u8 = (255.999 * color).clamp(Vec3::ZERO, Vec3::splat(255.0));
                writer
                    .write_all(
                        format!(
                            "\n{} {} {}",
                            color_u8.x as u32, color_u8.y as u32, color_u8.z as u32
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                pb.inc(1);
            }
        }
        pb.finish();
        multi.remove(&pb);
    }
}
