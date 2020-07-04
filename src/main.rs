mod aabb;
mod camera;
mod hittable;
mod image_gen;
mod material;
mod ray;
mod raytrace;
mod render;
mod utils;

use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::image_gen::ImageProvider;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::raytrace::{vec3, RayTracer};
use crate::render::Renderer;
use miniquad::*;
use std::time::Instant;
use ultraviolet::Vec3;

type HittableList = Vec<Box<dyn Hittable>>;

struct Stage {
    renderer: Renderer,
    provider: ImageProvider,
    tracer: RayTracer<HittableList>,
    last: Option<(f32, f32)>,
}

fn random_scene() -> HittableList {
    let mut world: HittableList = vec![];

    world.push(Box::new(Sphere {
        center: vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Lambertian::new(Vec3::broadcast(0.5)),
    }));

    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = rand::random::<f32>();
            let center = vec3(
                a as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - vec3(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::from(rand::random::<[f32; 3]>())
                        * Vec3::from(rand::random::<[f32; 3]>());
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Lambertian::new(albedo),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo =
                        Vec3::broadcast(0.5) + Vec3::from(rand::random::<[f32; 3]>()) * 0.5;
                    let fuzz = rand::random::<f32>() * 0.5;
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Metal::new(albedo, fuzz),
                    }));
                } else {
                    // glass
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Dielectric::new(1.5),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere {
        center: vec3(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Dielectric::new(1.5),
    }));

    world.push(Box::new(Sphere {
        center: vec3(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Lambertian::new(vec3(0.4, 0.2, 0.1)),
    }));

    world.push(Box::new(Sphere {
        center: vec3(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Metal::new(vec3(0.7, 0.6, 0.5), 0.0),
    }));

    return world;
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let world = random_scene();

        Stage {
            renderer: Renderer::new(ctx),
            provider: ImageProvider::new(),
            tracer: RayTracer::new(world),
            last: None,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (w, h) = ctx.screen_size();

        let t = Instant::now();
        let pixels = self
            .provider
            .get_next(&mut self.tracer, w as usize, h as usize);
        print!("Image gen took {}ms\n", t.elapsed().as_secs_f32() * 1000.0);

        self.renderer.draw_pixels(ctx, pixels);
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        const SPEED: f32 = 0.16;
        match keycode {
            KeyCode::Right | KeyCode::D => {
                self.tracer.cam.right(SPEED);
                self.provider.moved();
            }
            KeyCode::Left | KeyCode::A => {
                self.tracer.cam.right(-SPEED);
                self.provider.moved();
            }
            KeyCode::Up | KeyCode::W => {
                self.tracer.cam.forward(SPEED);
                self.provider.moved();
            }
            KeyCode::Down | KeyCode::S => {
                self.tracer.cam.forward(-SPEED);
                self.provider.moved();
            }
            KeyCode::Space => {
                self.tracer.cam.up(SPEED);
                self.provider.moved();
            }
            KeyCode::LeftShift => {
                self.tracer.cam.up(-SPEED);
                self.provider.moved();
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn mouse_motion_event(&mut self, _: &mut Context, x: f32, y: f32) {
        if let Some((lx, ly)) = self.last {
            self.tracer.cam.eye_horiz(0.003 * (x - lx) as f32);
            self.tracer
                .cam
                .eye_vert(0.003 * (y - ly) as f32 * self.tracer.cam.aspect_ratio);
            self.provider.moved();
            self.last = Some((x, y));
        }
    }

    fn mouse_button_down_event(&mut self, _: &mut Context, _: MouseButton, x: f32, y: f32) {
        self.last = Some((x, y));
    }

    fn mouse_button_up_event(&mut self, _: &mut Context, _: MouseButton, _x: f32, _y: f32) {
        self.last = None;
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    conf.window_title = "raytrace".to_owned();
    conf.window_width = 1280 / 4;
    conf.window_height = 720 / 4;

    miniquad::start(conf, |mut ctx| UserData::owning(Stage::new(&mut ctx), ctx));
}
