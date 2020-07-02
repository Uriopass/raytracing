use crate::hittable::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub mat: T,
}

impl<T: Material> Hittable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.pos - self.center;

        let half_b = oc.dot(r.dir);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - c;
        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let mut t = -half_b - root;
        if t < t_min || t > t_max {
            t = -half_b + root;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        Some(Hit::new(r, p, outward_normal, t, &self.mat))
    }
}
