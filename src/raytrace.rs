use crate::camera::{Camera, RayGenerator};
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
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

const SAMPLES: usize = 8;

impl<T: Hittable + Sync> RayTracer<T> {
    pub fn new(world: T) -> Self {
        let cam = Camera::default();
        let gen = cam.ray_generator();
        Self { world, cam, gen }
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth > 0 {
            if let Some(hit) = self.world.hit(ray, 0.001, std::f32::INFINITY) {
                let bounce_dir = (hit.normal + random_unit_vector()).normalized();
                return 0.5 * self.ray_color(&Ray::new(hit.p, bounce_dir), depth - 1);
            }
        }

        let v = 0.5 * ray.dir.y + 0.5;
        (1.0 - v) * vec3(1.0, 1.0, 1.0) + v * vec3(0.5, 0.7, 1.0)
    }

    pub fn init(&mut self) {
        self.gen = self.cam.ray_generator();
    }

    pub fn get_pixel(&self, x: f32, y: f32, resx: f32, resy: f32) -> Color {
        let mut col = Color::zero();
        for _ in 0..SAMPLES {
            let ray = self.gen.ray(
                x + resx * rand::random::<f32>(),
                y + resy * rand::random::<f32>(),
            );
            col += self.ray_color(&ray, 4);
        }
        col / SAMPLES as f32
    }
}
