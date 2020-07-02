pub mod sphere;

use crate::ray::Ray;
use ordered_float::OrderedFloat;
use std::ops::Deref;
use ultraviolet::Vec3;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.deref().hit(ray, t_min, t_max)
    }
}

impl<T: Hittable> Hittable for &[T] {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.iter()
            .filter_map(move |x| x.hit(ray, t_min, t_max))
            .min_by_key(|rec| OrderedFloat(rec.t))
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.as_slice().hit(ray, t_min, t_max)
    }
}

pub struct Hit {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Hit {
    pub fn new(ray: &Ray, p: Vec3, outward_normal: Vec3, t: f32) -> Self {
        let front_face = ray.dir.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}
