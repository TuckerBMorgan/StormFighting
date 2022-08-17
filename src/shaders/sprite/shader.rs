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
    pub color_pallete_in: [std140::vec3;4],
    pub color_pallete_out: [std140::vec3;4]
    /*
    pub color_pallete_in: std140::mat4,
    pub color_pallete_out: std140::mat4
    */
}

impl SpriteUniform {
    pub fn new(ortho: Matrix4<f32>) -> SpriteUniform {
        let mut color_pallete_in = [std140::vec3::fill(0.96), std140::vec3::fill(0.96), std140::vec3::fill(0.96), std140::vec3::fill(0.96)];

        color_pallete_in[0].x = 246.0 / 255.0;
        color_pallete_in[0].y = 246.0 / 255.0;
        color_pallete_in[0].z = 246.0 / 255.0;

        color_pallete_in[1].x = 197.0 / 255.0;
        color_pallete_in[1].y = 197.0 / 255.0;
        color_pallete_in[1].z = 230.0 / 255.0;

        color_pallete_in[2].x = 230.0 / 255.0;
        color_pallete_in[2].y = 230.0 / 255.0;
        color_pallete_in[2].z = 213.0 / 255.0;

        color_pallete_in[3].x = 230.0 / 255.0;
        color_pallete_in[3].y = 213.0 / 255.0;
        color_pallete_in[3].z = 197.0 / 255.0;

        color_pallete_in[3].x = 180.0 / 255.0;
        color_pallete_in[3].y = 180.0 / 255.0;
        color_pallete_in[3].z = 180.0 / 255.0;

        let mut color_pallete_out = [std140::vec3::fill(0.10), std140::vec3::fill(0.20), std140::vec3::fill(0.10), std140::vec3::fill(0.10)];

        color_pallete_out[1].x = 49.0 / 255.0;
        color_pallete_out[1].y = 115.0 / 255.0;
        color_pallete_out[1].z = 131.0 / 255.0;

        color_pallete_out[1].x = 32.0 / 255.0;
        color_pallete_out[1].y = 82.0 / 255.0;
        color_pallete_out[1].z = 98.0 / 255.0;

        color_pallete_out[2].x = 49.0 / 255.0;
        color_pallete_out[2].y = 98.0 / 255.0;
        color_pallete_out[2].z = 115.0 / 255.0;

        color_pallete_out[3].x = 49.0 / 255.0;
        color_pallete_out[3].y = 32.0 / 255.0;
        color_pallete_out[3].z = 115.0 / 255.0;

        color_pallete_out[3].x = 0.0 / 255.0;
        color_pallete_out[3].y = 0.0 / 255.0;
        color_pallete_out[3].z = 10.0 / 255.0;

        SpriteUniform {
            ortho: ortho.into(),
            color_pallete_in,
            color_pallete_out
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
      //  self.shader.draw(DrawMode::TriangleStrip, uniform, [atlas], &[buffer]);
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
        shader.shader.bind(&self.uniform, [&self.atlas]);
        self.buffer.draw();
//        shader.draw(&self.uniform, &self.atlas, &self.scanline, &self.buffer);
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.uniform.set(SpriteUniform::new(transform));
    }
}
