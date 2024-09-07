use glam::Vec3;
use rand::random;

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(
            random::<f32>() * 2.0 - 1.0,
            random::<f32>() * 2.0 - 1.0,
            random::<f32>() * 2.0 - 1.0,
        );
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

pub fn linear_to_gamma(linear: Vec3) -> Vec3 {
    linear.map(|x| if x > 0.0 { x.sqrt() } else { x })
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
    r_out_perp + r_out_parallel
}

/// Schlick's approximation for reflectance
pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (ref_idx - 1.0) / (ref_idx + 1.0);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}