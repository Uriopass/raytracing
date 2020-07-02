use crate::ray::Ray;
use crate::raytrace::vec3;
use ultraviolet::{Rotor3, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub eye: Rotor3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: vec3(1.0, 0.0, 0.0),
            eye: Rotor3::identity(),
            up: Vec3::unit_y(),
            fov: 80.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }
}

pub struct RayGenerator {
    pos: Vec3,
    ll: Vec3,
    horiz: Vec3,
    vert: Vec3,
}

impl Camera {
    pub fn ray_generator(&self) -> RayGenerator {
        let theta = self.fov.to_radians();

        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = self.eye * Vec3::unit_z();
        let u = self.up.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = -horizontal / 2.0 - vertical / 2.0 - w;

        RayGenerator {
            pos: self.pos,
            ll: lower_left_corner,
            horiz: horizontal,
            vert: vertical,
        }
    }

    pub fn translate(&mut self, vec: Vec3) {
        self.pos += vec;
    }

    pub fn eye_horiz(&mut self, ang: f32) {
        self.eye = Rotor3::from_euler_angles(0.0, 0.0, ang as f32) * self.eye;
    }

    pub fn eye_vert(&mut self, ang: f32) {
        self.eye = Rotor3::from_euler_angles(0.0, -ang as f32, 0.0) * self.eye;
    }
}

impl RayGenerator {
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.pos,
            (self.ll + self.horiz * u + self.vert * v).normalized(),
        )
    }
}
