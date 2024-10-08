use glam::Vec3;
use rand::random;
use raytracing::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    Sphere, World,
};

// Ideal aspect ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
    // Setup world
    let mut world: World = vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ))];

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = random::<f32>();

                let albedo = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
                    * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>());
                let material: Box<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    Box::new(Metal::new(albedo).fuzz(random::<f32>() * 0.5))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                world.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5)).fuzz(0.0)),
    )));

    // Image
    let image_width = 1280;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let camera = Camera::new(aspect_ratio)
        .samples_per_pixel(500)
        .fov(20.0)
        .pos(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::ZERO)
        .focus_distance(10.0)
        .defocus_angle(0.6);

    camera.render_to_path(&world, image_width, "image.png");
}
