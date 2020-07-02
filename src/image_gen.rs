use crate::hittable::Hittable;
use crate::raytrace::RayTracer;
use rayon::prelude::*;

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

    pub fn get_next<T: Hittable + Sync>(
        &mut self,
        tracer: &mut RayTracer<T>,
        width: usize,
        height: usize,
    ) -> &[u8] {
        if width * height * 4 != self.img.len() {
            self.resize(width, height);
        }

        tracer.cam.aspect_ratio = (width as f32) / height as f32;
        tracer.init();

        let w = 1.0 / width as f32;
        let h = 1.0 / height as f32;

        let tracer = &tracer;
        self.img
            .as_mut_slice()
            .par_chunks_exact_mut(width * 4)
            .enumerate()
            .for_each(move |(y, line)| {
                for (x, rgba) in line.chunks_exact_mut(4).enumerate() {
                    let c = tracer.get_pixel(x as f32 * w, y as f32 * h, w, h, 1);

                    unsafe {
                        *rgba.get_unchecked_mut(0) = (c.x * 255.999) as u8;
                        *rgba.get_unchecked_mut(1) = (c.y * 255.999) as u8;
                        *rgba.get_unchecked_mut(2) = (c.z * 255.999) as u8;
                    }
                }
            });
        &self.img
    }
}
