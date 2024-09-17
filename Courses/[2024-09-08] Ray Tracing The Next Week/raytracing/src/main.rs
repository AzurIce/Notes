use std::sync::Arc;

use glam::Vec3;
use rand::random;
use raytracing::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    primitive::{Quad, Sphere},
    texture::{CheckerTexture, ImageTexture, SolidCheckerTexture, SolidColor, Texture},
    world::bvh::{AabbHittable, BvhNode},
};

// Ideal aspect ratio
const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn quads() -> impl AabbHittable + Send + Sync {
    let mut objects = Vec::new();

    objects.push(Box::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.2, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 1.0, 0.2)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 0.2, 1.0)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(1.0, 0.5, 0.0)),
        ))))),
    )));
    objects.push(Box::new(Quad::new(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.2, 0.8, 0.8)),
        ))))),
    )));

    let objects = objects
        .into_iter()
        .map(|obj| obj as Box<dyn AabbHittable + Send + Sync>)
        .collect();
    BvhNode::from_objects(objects)
}

fn world() -> impl AabbHittable + Send + Sync {
    let mut objects = Vec::new();
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidCheckerTexture::new(
                0.5,
                Arc::new(Box::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1)))),
                Arc::new(Box::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)))),
            ),
        ))))),
    )));

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
                let texture: Arc<Box<dyn Texture + Send + Sync>> =
                    Arc::new(Box::new(SolidColor::new(albedo)));
                let material: Box<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    Box::new(Lambertian::new(texture))
                } else if choose_mat < 0.95 {
                    Box::new(Metal::new(texture).fuzz(random::<f32>() * 0.5))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                objects.push(Box::new(Sphere::new(center, 0.2, Arc::new(material))));
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Box::new(Dielectric::new(1.5))),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Box::new(Lambertian::new(Arc::new(Box::new(
            SolidColor::new(Vec3::new(0.4, 0.2, 0.1)),
        ))))),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Box::new(
            Metal::new(Arc::new(Box::new(SolidColor::new(Vec3::new(
                0.7, 0.6, 0.5,
            )))))
            .fuzz(0.0),
        )),
    )));

    let objects = objects
        .into_iter()
        .map(|obj| obj as Box<dyn AabbHittable + Send + Sync>)
        .collect();
    BvhNode::from_objects(objects)
}

fn checkered_spheres() -> impl AabbHittable + Send + Sync {
    let mut objects = Vec::new();

    let earth_texture = ImageTexture::from_path("assets/earthmap.jpg");
    let earth_texture: Arc<Box<dyn Texture + Send + Sync>> = Arc::new(Box::new(earth_texture));
    let material: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Lambertian::new(earth_texture)));

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        material.clone(),
    )));

    let objects = objects
        .into_iter()
        .map(|obj| obj as Box<dyn AabbHittable + Send + Sync>)
        .collect();
    BvhNode::from_objects(objects)
}

fn main() {
    // Setup world

    // Image
    let image_width = 1280;
    let image_height = (image_width as f32 / ASPECT_RATIO) as usize;
    let aspect_ratio = image_width as f32 / image_height as f32; // real aspect ratio

    let camera = Camera::new(aspect_ratio)
        .samples_per_pixel(500)
        .fov(80.0)
        .pos(Vec3::new(0.0, 0.0, 9.0))
        .look_at(Vec3::ZERO)
        .focus_distance(10.0);

    // i9-9900k: cost: 419.357666s
    // let objects = objects
    //     .into_iter()
    //     .map(|obj| obj as Box<dyn Hittable + Send + Sync>)
    //     .collect();
    // let world = List::from_objects(objects);
    // camera.render_to_path(&world, image_width, "image.png");

    // i9-9900k: ramdom axis cost: 76.5858159s
    // i9-9900k: longest axis cost: 69.4391338s
    let world = quads();
    camera.render_to_path(&world, image_width, "image.png");
}
