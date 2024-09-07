use c08_gamma_correction::{camera::Camera, Sphere, World};
use glam::Vec3;

// Ideal aspect ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    // Setup world
    let world: World = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Image
    let image_width = 400;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    // let (image_width, image_height) = (400, 225);
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let camera = Camera::new(aspect_ratio);

    camera.render_to_path(&world, image_width, "image_c08.png");
}
