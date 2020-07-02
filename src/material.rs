use crate::hittable::Hit;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use ultraviolet::Vec3;

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material: Send + Sync {
    fn scatter<'a>(&self, ray: &Ray, hit: Hit<'a>) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _ray: &Ray, hit: Hit<'a>) -> Option<ScatterResult> {
        let bounce_dir = (hit.normal + random_unit_vector()).normalized();

        Some(ScatterResult {
            scattered: Ray::new(hit.p, bounce_dir),
            attenuation: self.albedo,
        })
    }
}
