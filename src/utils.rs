use crate::raytrace::vec3;
use ultraviolet::Vec3;

pub fn random_unit_vector() -> Vec3 {
    let a = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let z = rand::random::<f32>() * 2.0 - 1.0;
    let r = (1.0 - z * z).sqrt();
    return vec3(r * a.cos(), r * a.sin(), z);
}

pub fn random_in_sphere() -> Vec3 {
    loop {
        let p = Vec3::from(rand::random::<[f32; 3]>());
        if p.mag_sq() <= 1.0 {
            return p;
        }
    }
}

pub fn v_min(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x.min(b.x),
        y: a.y.min(b.y),
        z: a.z.min(b.z),
    }
}

pub fn v_max(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x.max(b.x),
        y: a.y.max(b.y),
        z: a.z.max(b.z),
    }
}
