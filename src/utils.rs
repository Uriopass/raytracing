use crate::raytrace::vec3;
use ultraviolet::Vec3;

pub fn random_unit_vector() -> Vec3 {
    let a = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let z = rand::random::<f32>() * 2.0 - 1.0;
    let r = (1.0 - z * z).sqrt();
    return vec3(r * a.cos(), r * a.sin(), z);
}
