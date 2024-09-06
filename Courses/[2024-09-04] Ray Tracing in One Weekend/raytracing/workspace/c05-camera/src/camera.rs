use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{log::logger, Hittable, Ray};
use ::log::info;
use glam::Vec3;
use indicatif::ProgressBar;

pub fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    if let Some(record) = world.hit(ray, 0.0, f32::INFINITY) {
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
}

impl Camera {
    pub fn new(camera_center: Vec3, focal_length: f32, aspect_ratio: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        Camera {
            focal_length,
            aspect_ratio,
            viewport_height,
            viewport_width,
            camera_center,
        }
    }

    pub fn render_to_ppm(
        &self,
        world: &Vec<Box<dyn Hittable>>,
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
        let pb = multi.add(ProgressBar::new(output_height as u64));
        for j in 0..output_height {
            for i in 0..output_width {
                let pixel_center =
                    pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
                let ray = Ray::new(self.camera_center, pixel_center - self.camera_center);

                let color = ray_color(&ray, world);
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
