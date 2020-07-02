use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::raytrace::{vec3, RayTracer};
use ultraviolet::Vec3;

pub struct ImageProvider {
    img: Vec<u8>,
    tracer: RayTracer<Vec<Box<dyn Hittable>>>,
}

impl ImageProvider {
    pub fn new() -> Self {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }));

        world.push(Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }));

        Self {
            img: vec![],
            tracer: RayTracer::new(world),
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
                let Vec3 { x: r, y: g, z: b } = self.tracer.get_pixel(
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
