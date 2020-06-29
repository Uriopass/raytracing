use crate::ray::Ray;
use ultraviolet::Vec3;

type Color = Vec3;

pub struct RayTracer {
    origin: Vec3,
    focal_length: f32,
}

impl RayTracer {
    pub fn new() -> Self {
        Self {
            origin: Vec3::zero(),
            focal_length: 1.0,
        }
    }

    fn ray_color(&self, ray: Ray) -> Color {
        if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        const BLUE: f32 = 0.6;
        let v = 0.5 * ray.dir.normalized().y + 0.5;
        Vec3::new(1.0 - v * BLUE, 1.0 - v * BLUE * 0.9, 1.0)
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        let ray = Ray {
            orig: self.origin,
            dir: Vec3::new(x, y, -self.focal_length) - self.origin,
        };

        self.ray_color(ray)
    }
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.orig - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}
