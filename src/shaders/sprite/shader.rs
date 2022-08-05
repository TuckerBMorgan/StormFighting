use storm::Context;
use storm::graphics::{Buffer, std140,DrawMode, Shader, ShaderDescriptor, Texture, Uniform, TextureFiltering
};
use crate::FighthingApp;
use crate::shaders::sprite::Sprite;
use cgmath::Matrix4;

impl ShaderDescriptor<1> for SpriteShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["texT"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = SpriteUniform;
    type VertexDescriptor = Sprite;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct SpriteUniform {
    pub ortho: std140::mat4,
}

impl SpriteUniform {
    pub fn new(ortho: Matrix4<f32>) -> SpriteUniform {
        SpriteUniform {
            ortho: ortho.into(),
        }
    }
}

pub struct SpriteShader {
    shader: Shader<SpriteShader, 1>,
}

impl SpriteShader {
    pub fn new(ctx: &mut Context<FighthingApp>) -> SpriteShader {
        SpriteShader {
            shader: Shader::new(ctx),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<SpriteUniform>, atlas: &Texture, _scanline: &Texture, buffer: &Buffer<Sprite>) {
        self.shader.draw(DrawMode::TriangleStrip, uniform, [atlas], &[buffer]);
    }
}

pub struct SpriteShaderPass {
    pub uniform: Uniform<SpriteUniform>,
    pub atlas: Texture,
    pub scanline: Texture,
    pub buffer: Buffer<Sprite>,
}

impl SpriteShaderPass {
    pub fn new(ortho: Matrix4<f32>, ctx: &mut Context<FighthingApp>) -> SpriteShaderPass {
        SpriteShaderPass {
            uniform: Uniform::new(ctx, SpriteUniform::new(ortho)),
            atlas: ctx.default_texture(),
            scanline: Texture::from_png(ctx, include_bytes!("../.../../../../resources/scanline_5.png"), TextureFiltering::none()),
            buffer: Buffer::new(ctx),
        }
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &SpriteShader) {
        shader.draw(&self.uniform, &self.atlas, &self.scanline, &self.buffer);
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.uniform.set(SpriteUniform::new(transform));
    }
}
