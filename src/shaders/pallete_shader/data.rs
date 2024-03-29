use storm::color::RGBA8;
use storm::graphics::{*};
use storm::graphics::VertexInstancing;
use storm::math::AABB2D;
use cgmath::*;

/// Configuration settings for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PalleteSprite {
    /// Position of the sprite. The X and Y coordinates represent the bottom left corner of the
    /// sprite. The Z coordinate represents sprite depth. Units are measured in pixels.
    pub pos: Vector3<f32>,
    /// Units are measured in pixels.
    pub size: Vector2<u16>,
    /// Texture to apply to the sprite. The default is a plain white texture.
    pub texture: TextureSection,
    /// Color multiplier to apply to the sprite. The default is white.
    pub color: RGBA8,
    /// Rotation of the sprite. Units are 1/65536th of a turn.
    pub rotation: u16,
    //Mapping for the scanline effects
    pub scanline_texture: TextureSection
}

impl VertexDescriptor for PalleteSprite {
    const INSTANCING: VertexInstancing = VertexInstancing::instanced(4);
    const ATTRIBUTES: &'static [VertexAttribute] = &[
        // Pos, Size, Texture, Color::RGBA8, Rotation
        VertexAttribute::new(3, VertexInputType::F32, VertexOutputType::F32),
        VertexAttribute::new(2, VertexInputType::U16, VertexOutputType::F32),
        VertexAttribute::new(4, VertexInputType::U16, VertexOutputType::NormalizedF32),
        VertexAttribute::new(4, VertexInputType::U8, VertexOutputType::NormalizedF32),
        VertexAttribute::new(1, VertexInputType::U16, VertexOutputType::NormalizedF32),
        VertexAttribute::new(4, VertexInputType::U16, VertexOutputType::NormalizedF32),
    ];
    const DRAW_MODE: DrawMode = DrawMode::TriangleStrip;
}

impl Default for PalleteSprite {
    fn default() -> PalleteSprite {
        PalleteSprite {
            pos: Vector3::new(0.0, 0.0, 0.0),
            size: Vector2::new(100, 100),
            texture: TextureSection::default(),
            color: RGBA8::WHITE,
            rotation: 0,
            scanline_texture: TextureSection::default()
        }
    }
}

impl PalleteSprite {
    /// Creates aa new sprite. This converts the rotation and size from floats automatically. Size
    /// is measured in pixels, and is limited to 65535. Rotation is measured in turns from [0, 1).
    /// Values outside of the range are wrapped into the range. For example, 1.75 is wrapped into
    /// 0.75, -0.4 is wrapped into 0.6.
    pub fn new(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        texture: TextureSection,
        color: RGBA8,
        rotation: f32,
        scanline_texture: TextureSection
    ) -> PalleteSprite {
        PalleteSprite {
            pos,
            size: {
                let x = (size.x as u32) & 0xFFFF;
                let y = (size.y as u32) & 0xFFFF;
                Vector2::new(x as u16, y as u16)
            },
            texture,
            color,
            rotation: (rotation.fract() * 65536.0) as u16,
            scanline_texture
        }
    }

    /// Creates a new sprite. This does not perform conversions and represents exactly the members
    /// of the sprite type.
    pub fn new_raw(
        pos: Vector3<f32>,
        size: Vector2<u16>,
        texture: TextureSection,
        color: RGBA8,
        rotation: u16,
        scanline_texture: TextureSection
    ) -> PalleteSprite {
        PalleteSprite {
            pos,
            size,
            texture,
            color,
            rotation,
            scanline_texture        
        }
    }
}

impl From<PalleteSprite> for AABB2D {
    fn from(sprite: PalleteSprite) -> Self {
        AABB2D::from_pos_size(sprite.pos.truncate(), sprite.size.cast().unwrap())
    }
}
