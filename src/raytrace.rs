use crate::ray::Ray;
use ultraviolet::Vec3;

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

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
        let sphere_pos = Vec3::new(0.0, 0.0, -1.0);
        let t = hit_sphere(sphere_pos, 0.5, &ray);
        if let Some(t) = t {
            let normal = (ray.at(t) - sphere_pos).normalized();
            return Vec3::broadcast(0.5) + normal * 0.5;
        }

        let v = 0.5 * ray.dir.normalized().y + 0.5;
        (1.0 - v) * vec3(1.0, 1.0, 1.0) + v * vec3(0.5, 0.7, 1.0)
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> Color {
        let ray = Ray {
            orig: self.origin,
            dir: Vec3::new(x, y, -self.focal_length) - self.origin,
        };

        self.ray_color(ray)
    }
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> Option<f32> {
    let oc = r.orig - center;
    let a = r.dir.mag_sq();
    let half_b = oc.dot(r.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}
