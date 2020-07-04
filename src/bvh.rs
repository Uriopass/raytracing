use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use rand::prelude::SliceRandom;

enum BVHNode {
    Leaf(usize),
    Branch {
        bbox: AABB,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

impl BVHNode {
    fn new(objects: &mut [Box<dyn Hittable>], offset: usize) -> (Self, AABB) {
        match objects {
            [] => panic!("empty node"),
            [a] => (Self::Leaf(offset), a.bbox().unwrap()),
            [a, b] => {
                let bbox = a.bbox().unwrap().extend(&b.bbox().unwrap());
                (BVHNode::Branch {
                    left: Box::new(Self::Leaf(offset)),
                    right: Box::new(Self::Leaf(offset+1)),
                    bbox,
                }, bbox)
            }
            xs => {
                let axis = *[0 as usize, 1, 2].choose(&mut rand::thread_rng()).unwrap();
                xs.sort_by(move |a, b| {
                    let abb = a.bbox().unwrap();
                    let bbb = b.bbox().unwrap();
                    abb.lo[axis].partial_cmp(&bbb.lo[axis]).unwrap()
                });

                let l = xs.len();
                let (left, right) = xs.split_at_mut(xs.len() / 2);

                let off_right = offset + left.len();

                let ((left, lbb), (right, rbb)) = if l > 50 {
                    rayon::join(|| Self::new(left, offset), || Self::new(right, off_right))
                } else {
                    (Self::new(left, offset), Self::new(right, off_right))
                };

                let bbox = lbb.extend(&rbb);
                (BVHNode::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox,
                }, bbox)
            }
        }
    }

    pub fn bbox(&self, objs: &[Box<dyn Hittable>]) -> AABB {
        match &self {
            BVHNode::Leaf(i) => objs[*i].bbox().unwrap(),
            BVHNode::Branch { bbox, .. } => *bbox,
        }
    }

    pub fn hit<'a>(
        &'a self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        objs: &'a [Box<dyn Hittable>],
    ) -> Option<Hit<'a>> {
        if !self.bbox(objs).hit(ray, t_min, t_max) || t_max <= t_min {
            return None;
        }

        match self {
            &BVHNode::Leaf(i) => unsafe { objs.get_unchecked(i) }.hit(ray, t_min, t_max),
            BVHNode::Branch { left, right, .. } => {
                let hit1 = left.hit(ray, t_min, t_max, objs);
                let hit2 = right.hit(ray, t_min, hit1.as_ref().map_or(t_max, |h| h.t), objs);

                hit2.or(hit1)
            }
        }
    }
}

pub struct BVH {
    objects: Vec<Box<dyn Hittable>>,
    node: Option<BVHNode>,
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        if objects.is_empty() {
            return Self {
                objects,
                node: None,
            };
        }

        let (node, _) = BVHNode::new(&mut objects, 0);
        Self {
            objects,
            node: Some(node),
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match &self.node {
            Some(x) => x.hit(ray, t_min, t_max, &self.objects),
            None => None,
        }
    }

    fn bbox(&self) -> Option<AABB> {
        let objs = &self.objects;
        self.node.as_ref().map(|x| x.bbox(objs))
    }
}
