use crate::ray::Ray;
use crate::utils::{v_max, v_min};
use ultraviolet::Vec3;

#[derive(Clone, Copy)]
pub struct AABB {
    pub lo: Vec3,
    pub hi: Vec3,
}

impl AABB {
    pub fn new(lo: Vec3, hi: Vec3) -> Self {
        Self { lo, hi }
    }

    pub fn empty() -> Self {
        Self {
            lo: Vec3::broadcast(0.0),
            hi: Vec3::broadcast(0.0),
        }
    }

    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self.lo[a] - r.pos[a]) * inv_d;
            let mut t1 = (self.hi[a] - r.pos[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn extend(&self, other: &AABB) -> AABB {
        Self {
            lo: v_min(self.lo, other.lo),
            hi: v_max(self.hi, other.hi),
        }
    }
}
