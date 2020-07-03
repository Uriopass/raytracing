use crate::hittable::Hittable;
use crate::raytrace::RayTracer;
use rayon::prelude::*;

pub struct ImageProvider {
    acc: Vec<u16>,
    pixels: Vec<u8>,
    samples: u32,
}

impl ImageProvider {
    pub fn new() -> Self {
        Self {
            acc: vec![],
            pixels: vec![],
            samples: 0,
        }
    }

    pub fn moved(&mut self) {
        for v in &mut self.acc {
            *v = 0;
        }
        self.samples = 0;
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.acc = vec![0; width * height * 3];
        self.pixels = vec![0xFF; width * height * 4];
        self.samples = 0;
        println!("\nresized to {} {}", width, height)
    }

    pub fn get_next<T: Hittable + Sync>(
        &mut self,
        tracer: &mut RayTracer<T>,
        width: usize,
        height: usize,
    ) -> &[u8] {
        if width * height * 4 != self.pixels.len() {
            self.resize(width, height);
        }

        tracer.cam.aspect_ratio = (width as f32) / height as f32;
        tracer.init();

        let w = 1.0 / width as f32;
        let h = 1.0 / height as f32;

        let tracer = &tracer;
        self.samples += 1;
        let samples = self.samples as u16;

        self.acc
            .as_mut_slice()
            .par_chunks_exact_mut(width * 3)
            .zip(self.pixels.par_chunks_exact_mut(width * 4))
            .enumerate()
            .for_each(move |(y, (acc, line))| {
                for (x, (rgb, rgba)) in acc
                    .chunks_exact_mut(3)
                    .zip(line.chunks_exact_mut(4))
                    .enumerate()
                {
                    let c = tracer.get_sample(x as f32 * w, y as f32 * h, w, h);

                    unsafe {
                        *rgb.get_unchecked_mut(0) += (c.x.sqrt() * 255.999) as u16;
                        *rgb.get_unchecked_mut(1) += (c.y.sqrt() * 255.999) as u16;
                        *rgb.get_unchecked_mut(2) += (c.z.sqrt() * 255.999) as u16;

                        *rgba.get_unchecked_mut(0) = (rgb.get_unchecked(0) / samples) as u8;
                        *rgba.get_unchecked_mut(1) = (rgb.get_unchecked(1) / samples) as u8;
                        *rgba.get_unchecked_mut(2) = (rgb.get_unchecked(2) / samples) as u8;
                    }
                }
            });

        &self.pixels
    }
}
