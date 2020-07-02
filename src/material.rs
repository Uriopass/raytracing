use crate::hittable::Hit;
use crate::ray::Ray;
use crate::utils::{random_in_sphere, random_unit_vector};
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

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter<'a>(&self, ray: &Ray, hit: Hit<'a>) -> Option<ScatterResult> {
        let reflected = ray.dir.reflected(hit.normal);

        if reflected.dot(hit.normal) > 0.0 {
            Some(ScatterResult {
                scattered: Ray::new(
                    hit.p,
                    (reflected + self.fuzz * random_in_sphere()).normalized(),
                ),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.mag_sq()).sqrt() * n;
    return r_out_parallel + r_out_perp;
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5);
}

impl Material for Dielectric {
    fn scatter<'a>(&self, ray: &Ray, hit: Hit<'a>) -> Option<ScatterResult> {
        let eta = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let cos_theta = -ray.dir.dot(hit.normal);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        if eta * sin_theta > 1.0 {
            let reflected = ray.dir.reflected(hit.normal);
            return Some(ScatterResult {
                scattered: Ray::new(hit.p, reflected),
                attenuation: Vec3::one(),
            });
        }

        let reflect_prob = schlick(cos_theta, self.ref_idx);
        if rand::random::<f32>() < reflect_prob {
            let reflected = ray.dir.reflected(hit.normal);
            return Some(ScatterResult {
                scattered: Ray::new(hit.p, reflected),
                attenuation: Vec3::one(),
            });
        }

        let refracted = refract(ray.dir, hit.normal, eta);

        Some(ScatterResult {
            scattered: Ray::new(hit.p, refracted),
            attenuation: Vec3::one(),
        })
    }
}
