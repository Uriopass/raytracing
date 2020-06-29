use crate::raytrace::RayTracer;

pub struct ImageProvider {
    img: Vec<u8>,
    tracer: RayTracer,
}

impl ImageProvider {
    pub fn new() -> Self {
        Self {
            img: vec![],
            tracer: RayTracer::new(),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.img = vec![0xFF; width * height * 4];
        println!("\nresized to {} {}", width, height)
    }

    pub fn get_next(&mut self, width: usize, height: usize) -> &[u8] {
        if width * height * 4 != self.img.len() {
            self.resize(width, height);
        }

        let aspect_ratio = width as f32 / height as f32;

        let img = &mut self.img[0..width * height * 4];

        let mut p = 0;
        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = self.tracer.get_pixel(
                    (x as f32 / width as f32) * 2.0 * aspect_ratio - aspect_ratio,
                    (y as f32 / height as f32) * 2.0 - 1.0,
                );

                img[p] = (r * 255.999) as u8;
                img[p + 1] = (g * 255.999) as u8;
                img[p + 2] = (b * 255.999) as u8;

                p += 4;
            }
        }
        img
    }
}
