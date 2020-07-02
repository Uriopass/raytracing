use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, Texture,
    VertexAttribute, VertexFormat,
};

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

pub struct Renderer {
    pipeline: Pipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl Renderer {
    pub fn new(ctx: &mut Context) -> Self {
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

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn draw_pixels(&mut self, ctx: &mut Context, pixels: &[u8]) {
        let (w, h) = ctx.screen_size();
        let w = w as u16;
        let h = h as u16;

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
