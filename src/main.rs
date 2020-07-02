mod camera;
mod hittable;
mod image_gen;
mod ray;
mod raytrace;
mod render;
mod utils;

use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::image_gen::ImageProvider;
use crate::raytrace::{vec3, RayTracer};
use crate::render::Renderer;
use miniquad::*;
use std::time::Instant;

struct Stage {
    renderer: Renderer,
    provider: ImageProvider,
    tracer: RayTracer<Vec<Box<dyn Hittable>>>,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let mut world: Vec<Box<dyn Hittable>> = vec![];

        world.push(Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }));

        world.push(Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }));

        Stage {
            renderer: Renderer::new(ctx),
            provider: ImageProvider::new(),
            tracer: RayTracer::new(world),
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

    fn mouse_motion_event(&mut self, _: &mut Context, x: f32, y: f32) {
        self.tracer.cam.eye_horiz(x * 0.01);
        self.tracer.cam.eye_vert(y * 0.01);
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    conf.window_title = "raytrace".to_owned();
    conf.window_width = 1280;
    conf.window_height = 720;

    miniquad::start(conf, |mut ctx| UserData::owning(Stage::new(&mut ctx), ctx));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;

    uniform vec2 offset;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub const META: ShaderMeta = ShaderMeta {
        images: &["tex"],
        uniforms: UniformBlockLayout {
            uniforms: &[UniformDesc::new("offset", UniformType::Float2)],
        },
    };

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
    }
}
