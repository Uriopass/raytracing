use crate::hittable::Hittable;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

type Color = Vec3;

pub struct RayTracer<T: Hittable> {
    origin: Vec3,
    focal_length: f32,
    world: T,
}

impl<T: Hittable> RayTracer<T> {
    pub fn new(world: T) -> Self {
        Self {
            origin: Vec3::zero(),
            focal_length: 1.0,
            world,
        }
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        if let Some(hit) = self.world.hit(ray, 0.0, std::f32::INFINITY) {
            return Vec3::broadcast(0.5) + hit.normal * 0.5;
        }

        let v = 0.5 * ray.dir.normalized().y + 0.5;
        (1.0 - v) * vec3(1.0, 1.0, 1.0) + v * vec3(0.5, 0.7, 1.0)
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        let ray = Ray {
            orig: self.origin,
            dir: Vec3::new(x, y, -self.focal_length) - self.origin,
        };

        self.ray_color(&ray)
    }
}
