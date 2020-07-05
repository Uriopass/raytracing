use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable};
use crate::ray::Ray;
use rand::prelude::SliceRandom;

enum BVHNodeKind {
    Leaf(usize),
    Branch {
        left: Box<BVHNode>,
        right: Box<BVHNode>,
    },
}

struct BVHNode {
    kind: BVHNodeKind,
    bbox: AABB,
}

impl BVHNode {
    fn new(objects: &mut [Box<dyn Hittable>], offset: usize) -> Self {
        match objects {
            [] => panic!("empty node"),
            [a] => Self {
                kind: BVHNodeKind::Leaf(offset),
                bbox: a.bbox().unwrap(),
            },
            [a, b] => {
                let abb = a.bbox().unwrap();
                let bbb = b.bbox().unwrap();
                let bbox = abb.extend(&bbb);
                Self {
                   kind: BVHNodeKind::Branch {
                       left: Box::new(BVHNode {
                           kind: BVHNodeKind::Leaf(offset),
                           bbox: abb,
                       }),
                       right: Box::new(BVHNode {
                           kind: BVHNodeKind::Leaf(offset + 1),
                           bbox: bbb,
                       }),
                   },
                    bbox
                }
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

                let (left, right) = if l > 50 {
                    rayon::join(|| Self::new(left, offset), || Self::new(right, off_right))
                } else {
                    (Self::new(left, offset), Self::new(right, off_right))
                };

                let bbox = left.bbox.extend(&right.bbox);
                Self {
                    kind: BVHNodeKind::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox
                }
            }
        }
    }

    pub fn hit<'a>(
        &'a self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        objs: &'a [Box<dyn Hittable>],
    ) -> Option<Hit<'a>> {
        if !self.bbox.hit(ray, t_min, t_max) || t_max <= t_min {
            return None;
        }

        match &self.kind {
            &BVHNodeKind::Leaf(i) => unsafe { objs.get_unchecked(i) }.hit(ray, t_min, t_max),
            BVHNodeKind::Branch { left, right, .. } => {
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

        let node = BVHNode::new(&mut objects, 0);
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
        self.node.as_ref().map(|x| x.bbox)
    }
}
