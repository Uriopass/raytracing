use ultraviolet::Vec3;

pub struct RayTracer;

impl RayTracer {
    pub fn new() -> Self {
        Self
    }

    pub fn get_pixel(&self, x: f32, y: f32) -> (f32, f32, f32) {
        let v = Vec3::new(1.0, 1.0, 1.0);
        (x, y, 0.0)
    }
}
