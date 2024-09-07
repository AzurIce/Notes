use c11_camera_pro::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    Sphere, World,
};
use glam::Vec3;

// Ideal aspect ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    // Setup world
    let world: World = vec![
        // Middle lambertian sphere
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        // Metal spheres
        Box::new(Sphere::new(
            Vec3::new(0.7, 0.0, -1.5),
            0.35,
            Box::new(Metal::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.2, -0.4, -0.6),
            0.1,
            Box::new(Metal::new(Vec3::new(1.0, 1.0, 1.0)).fuzz(0.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.3, 0.0, -0.4),
            0.15,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.3, 0.0, -0.4),
            0.15,
            Box::new(Dielectric::new(1.0 / 1.33)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.1, 0.3, -0.3),
            0.25,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.1, 0.3, -0.3),
            0.2,
            Box::new(Dielectric::new(1.0 / 1.5)),
        )),
        // Ground
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
    ];

    // Image
    // let image_width = 400;
    let image_width = 1280;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    // let (image_width, image_height) = (400, 225);
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let mut camera = Camera::new(aspect_ratio);

    camera.render_to_path(&world, image_width, "image_c11-fov90.png");

    camera.set_fov(30.0);
    camera.render_to_path(&world, image_width, "image_c11-fov30.png");

    camera.set_fov(150.0);
    camera.render_to_path(&world, image_width, "image_c11-fov150.png");
}
