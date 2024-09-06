use std::{
    fs::File,
    io::{BufWriter, Write},
};

use c02_ray::Ray;
use env_logger::Env;
use glam::Vec3;
use indicatif::{MultiProgress, ProgressBar};
use indicatif_log_bridge::LogWrapper;
use log::info;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0); // 从 [-1, 1] 映射到 [0, 1]
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let logger = env_logger::Builder::from_env(Env::default().default_filter_or("info")).build();
    let level = logger.filter();
    let multi = MultiProgress::new();

    LogWrapper::new(multi.clone(), logger).try_init().unwrap();
    log::set_max_level(level);

    // Image
    let image_width = 400;
    let image_height = ((image_width as f32 / ASPECT_RATIO) as u32).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width =
        (viewport_height as f32 * (image_width as f32 / image_height as f32)) as u32;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f32, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let file = File::create("image_c02.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    // Render
    writer
        .write_all(format!("P3\n{} {}\n255", image_width, image_height).as_bytes())
        .unwrap();

    info!("generating image");
    let pg = multi.add(ProgressBar::new((image_height * image_width).into()));
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray = Ray::new(camera_center, pixel_center - camera_center);

            let color = ray_color(&ray);
            let color_u8 = (255.999 * color).clamp(Vec3::ZERO, Vec3::splat(255.0));
            writer
                .write_all(format!("\n{} {} {}", color_u8.x, color_u8.y, color_u8.z).as_bytes())
                .unwrap();
            pg.inc(1);
        }
    }
    pg.finish();
    multi.remove(&pg);
}
