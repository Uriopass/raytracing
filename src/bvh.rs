use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use rand::prelude::SliceRandom;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = *[0 as usize, 1, 2].choose(&mut rand::thread_rng()).unwrap();

        match objects {
            [] => Self {
                left: Arc::new(()),
                right: Arc::new(()),
                bbox: AABB::empty(),
            },
            [a] => Self {
                left: a.clone(),
                right: Arc::new(()),
                bbox: a.bbox().unwrap(),
            },
            [a, b] => Self {
                left: a.clone(),
                right: b.clone(),
                bbox: a.bbox().unwrap().extend(&b.bbox().unwrap()),
            },
            xs => {
                xs.sort_by(move |a, b| {
                    let abb = a.bbox().unwrap();
                    let bbb = b.bbox().unwrap();
                    abb.lo[axis].partial_cmp(&bbb.lo[axis]).unwrap()
                });

                let l = xs.len();
                let (left, right) = xs.split_at_mut(xs.len() / 2);

                let (left, right) = if l > 50 {
                    rayon::join(|| BVHNode::new(left), || BVHNode::new(right))
                } else {
                    (BVHNode::new(left), BVHNode::new(right))
                };

                let bbox = left.bbox().unwrap().extend(&right.bbox().unwrap());
                Self {
                    left: Arc::new(left),
                    right: Arc::new(right),
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let hit1 = self.left.hit(ray, t_min, t_max);
        let hit2 = self
            .right
            .hit(ray, t_min, hit1.as_ref().map_or(t_max, |h| h.t));

        hit2.or(hit1)
    }

    fn bbox(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
