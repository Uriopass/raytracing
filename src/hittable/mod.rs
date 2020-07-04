pub mod sphere;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use ordered_float::OrderedFloat;
use std::ops::Deref;
use std::sync::Arc;
use ultraviolet::Vec3;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bbox(&self) -> Option<AABB>;
}

impl Hittable for () {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<Hit> {
        None
    }

    fn bbox(&self) -> Option<AABB> {
        Some(AABB::empty())
    }
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.deref().hit(ray, t_min, t_max)
    }

    fn bbox(&self) -> Option<AABB> {
        self.deref().bbox()
    }
}

impl Hittable for Arc<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.deref().hit(ray, t_min, t_max)
    }

    fn bbox(&self) -> Option<AABB> {
        self.deref().bbox()
    }
}

impl<T: Hittable> Hittable for &[T] {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.iter()
            .filter_map(move |x| x.hit(ray, t_min, t_max))
            .min_by_key(|rec| OrderedFloat(rec.t))
    }

    fn bbox(&self) -> Option<AABB> {
        let mut bbox = self.first()?.bbox()?;
        for obj in self.iter().skip(1) {
            let bbox_o = obj.bbox()?;
            bbox = bbox.extend(&bbox_o);
        }
        Some(bbox)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.iter()
            .filter_map(move |x| x.hit(ray, t_min, t_max))
            .min_by_key(|rec| OrderedFloat(rec.t))
    }

    fn bbox(&self) -> Option<AABB> {
        self.as_slice().bbox()
    }
}

pub struct Hit<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn new(ray: &Ray, p: Vec3, outward_normal: Vec3, t: f32, mat: &'a dyn Material) -> Self {
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
            mat,
        }
    }
}
