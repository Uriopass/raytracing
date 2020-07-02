use crate::hittable::Hittable;
use crate::raytrace::RayTracer;

pub struct ImageProvider {
    img: Vec<u8>,
}

impl ImageProvider {
    pub fn new() -> Self {
        Self { img: vec![] }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.img = vec![0xFF; width * height * 4];
        println!("\nresized to {} {}", width, height)
    }

    pub fn get_next<T: Hittable>(
        &mut self,
        tracer: &mut RayTracer<T>,
        width: usize,
        height: usize,
    ) -> &[u8] {
        if width * height * 4 != self.img.len() {
            self.resize(width, height);
        }

        let img = &mut self.img[0..width * height * 4];

        tracer.cam.aspect_ratio = (width as f32) / height as f32;
        tracer.init();

        let mut p = 0;
        for y in 0..height {
            for x in 0..width {
                let c = tracer.get_pixel(x as f32 / width as f32, y as f32 / height as f32, 1);

                img[p] = (c.x * 255.999) as u8; // r
                img[p + 1] = (c.y * 255.999) as u8; // g
                img[p + 2] = (c.z * 255.999) as u8; // b

                p += 4;
            }
        }
        img
    }
}
