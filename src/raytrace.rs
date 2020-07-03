use crate::camera::{Camera, RayGenerator};
use crate::hittable::Hittable;
use crate::material::ScatterResult;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

type Color = Vec3;

pub struct RayTracer<T: Hittable + Sync> {
    pub cam: Camera,
    gen: RayGenerator,
    world: T,
}

impl<T: Hittable + Sync> RayTracer<T> {
    pub fn new(world: T) -> Self {
        let cam = Camera::default();
        let gen = cam.ray_generator();
        Self { world, cam, gen }
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth > 0 {
            if let Some(hit) = self.world.hit(ray, 0.001, std::f32::INFINITY) {
                if let Some(ScatterResult {
                    scattered,
                    attenuation,
                }) = hit.mat.scatter(ray, hit)
                {
                    return attenuation * self.ray_color(&scattered, depth - 1);
                }
                return Vec3::zero();
            }
        } else {
            return Vec3::zero();
        }

        let v = 0.5 * ray.dir.y + 0.5;
        (1.0 - v) * vec3(1.0, 1.0, 1.0) + v * vec3(0.5, 0.7, 1.0)
    }

    pub fn init(&mut self) {
        self.gen = self.cam.ray_generator();
    }

    pub fn get_sample(&self, x: f32, y: f32, resx: f32, resy: f32) -> Color {
        let ray = self.gen.ray(
            x + resx * rand::random::<f32>(),
            y + resy * rand::random::<f32>(),
        );
        self.ray_color(&ray, 4)
    }
}
