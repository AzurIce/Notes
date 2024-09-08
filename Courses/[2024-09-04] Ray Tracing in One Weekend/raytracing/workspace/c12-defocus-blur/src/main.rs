use c12_defocus_blur::{
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

    // From the book
    // let world: World = vec![
    //     Box::new(Sphere::new(
    //         Vec3::new(0.0, -100.5, -1.0),
    //         100.0,
    //         Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    //     )),
    //     Box::new(Sphere::new(
    //         Vec3::new(0.0, 0.0, -1.2),
    //         0.5,
    //         Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    //     )),
    //     Box::new(Sphere::new(
    //         Vec3::new(-1.0, 0.0, -1.0),
    //         0.5,
    //         Box::new(Dielectric::new(1.5)),
    //     )),
    //     Box::new(Sphere::new(
    //         Vec3::new(-1.0, 0.0, -1.0),
    //         0.4,
    //         Box::new(Dielectric::new(1.0 / 1.5)),
    //     )),
    //     Box::new(Sphere::new(
    //         Vec3::new(1.0, 0.0, -1.0),
    //         0.5,
    //         Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)).fuzz(1.0)),
    //     )),
    // ];

    // Image
    // let image_width = 400;
    let image_width = 1280;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    // let (image_width, image_height) = (400, 225);
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let camera = Camera::new(aspect_ratio)
        .fov(30.0)
        .pos(Vec3::new(0.6, 0.3, 1.3))
        .look_at(Vec3::new(0.2, 0.0, -0.64))
        .focus_distance(1.7)
        .defocus_angle(10.0);
    println!("camera focus at: {}", camera.focus_point());

    camera.render_to_path(&world, image_width, "image_c12.png");
}
