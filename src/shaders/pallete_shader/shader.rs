use storm::Context;
use storm::graphics::{Buffer, std140, Shader, ShaderDescriptor, Texture, Uniform, TextureFiltering
};
use crate::FighthingApp;
use crate::shaders::{PalleteSprite};
use cgmath::Matrix4;

impl ShaderDescriptor<1> for PalleteSpriteShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("pallete_fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["texT"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = PalleteSpriteUniform;
    type VertexDescriptor = PalleteSprite;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct PalleteSpriteUniform {
    pub ortho: std140::mat4,
    pub pallete: [std140::vec3;256],
}

impl PalleteSpriteUniform {
    pub fn new(ortho: Matrix4<f32>, pallete: [cgmath::Vector3<f32>;256]) -> PalleteSpriteUniform {
        let mut test : [std140::vec3; 256] = [std140::vec3::zero(); 256];
        for (index, element) in pallete.iter().enumerate() {
            test[index].x = element.x;
            test[index].y = element.y;
            test[index].z = element.z;
        }
        PalleteSpriteUniform {
            ortho: ortho.into(),
            pallete: test
        }
    }
}

pub struct PalleteSpriteShader {
    shader: Shader<PalleteSpriteShader, 1>,
}

impl PalleteSpriteShader {
    pub fn new(ctx: &mut Context<FighthingApp>) -> PalleteSpriteShader {
        PalleteSpriteShader {
            shader: Shader::new(ctx),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, _uniform: &Uniform<PalleteSpriteUniform>, _atlas: &Texture, _scanline: &Texture, _buffer: &Buffer<PalleteSprite>) {
      //  self.shader.draw(DrawMode::TriangleStrip, uniform, [atlas], &[buffer]);
    }
}

pub struct PalleteSpriteShaderPass {
    pub uniform: Uniform<PalleteSpriteUniform>,
    pub atlas: Texture,
    pub scanline: Texture,
    pub buffer: Buffer<PalleteSprite>,
    pub pallete: [cgmath::Vector3<f32>;256]
}

impl PalleteSpriteShaderPass {
    pub fn new(ortho: Matrix4<f32>, ctx: &mut Context<FighthingApp>, pallete: [cgmath::Vector3<f32>;256]) -> PalleteSpriteShaderPass {
        PalleteSpriteShaderPass {
            uniform: Uniform::new(ctx, PalleteSpriteUniform::new(ortho, pallete)),
            atlas: ctx.default_texture(),
            scanline: Texture::from_png(ctx, include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),  "/resources/scanline_5.png")), TextureFiltering::none()),
            buffer: Buffer::new(ctx),
            pallete
        }
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &PalleteSpriteShader) {
        shader.shader.bind(&self.uniform, [&self.atlas]);
        self.buffer.draw();
//        shader.draw(&self.uniform, &self.atlas, &self.scanline, &self.buffer);
    }

    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.uniform.set(PalleteSpriteUniform::new(transform, self.pallete));
    }
}
