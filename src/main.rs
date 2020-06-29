#![allow(dead_code)]

mod image_gen;
mod ray;
mod raytrace;

use crate::image_gen::ImageProvider;
use miniquad::*;
use std::time::Instant;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    provider: ImageProvider,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::META).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage {
            vertex_buffer,
            index_buffer,
            pipeline,
            provider: ImageProvider::new(),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (w, h) = ctx.screen_size();
        let w = w as u16;
        let h = h as u16;

        let t = Instant::now();
        let pixels = self.provider.get_next(w as usize, h as usize);
        print!("\rImage gen took {}ms", t.elapsed().as_secs_f32() * 1000.0);

        let texture = Texture::from_rgba8(ctx, w, h, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![self.vertex_buffer],
            index_buffer: self.index_buffer,
            images: vec![texture],
        };

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&bindings);

        ctx.apply_uniforms(&shader::Uniforms { offset: (0.0, 0.0) });

        ctx.draw(0, self.index_buffer.size() as i32, 1);

        ctx.end_render_pass();

        ctx.commit_frame();
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
