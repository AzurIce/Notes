use glam::Vec3;
use rand::random;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0, random::<f32>() * 2.0 - 1.0);
        if p.length_squared() >= f32::EPSILON && p.length_squared() - 1.0 <= f32::EPSILON {
            return p;
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let p = random_in_unit_sphere().normalize();
    if p.dot(normal) > 0.0 {
        p
    } else {
        -p
    }
}