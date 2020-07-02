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
use crate::material::Lambertian;
use crate::raytrace::{vec3, RayTracer};
use crate::render::Renderer;
use miniquad::*;
use std::time::Instant;
use ultraviolet::Vec3;

struct Stage {
    renderer: Renderer,
    provider: ImageProvider,
    tracer: RayTracer<Vec<Box<dyn Hittable>>>,
    last: Option<(f32, f32)>,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: vec3(0.5, 0.0, -1.0),
            radius: 0.5,
            mat: Lambertian::new(Vec3::unit_x()),
        }));

        world.push(Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,

            mat: Lambertian::new(Vec3::broadcast(1.0)),
        }));

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
            }
            KeyCode::Left | KeyCode::A => {
                self.tracer.cam.right(-SPEED);
            }
            KeyCode::Up | KeyCode::W => {
                self.tracer.cam.forward(SPEED);
            }
            KeyCode::Down | KeyCode::S => {
                self.tracer.cam.forward(-SPEED);
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
    conf.window_width = 1280;
    conf.window_height = 720;

    miniquad::start(conf, |mut ctx| UserData::owning(Stage::new(&mut ctx), ctx));
}
