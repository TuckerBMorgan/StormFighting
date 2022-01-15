use core::convert::{From};

use storm::color::RGBA8;
use storm::cgmath::{Vector2, Vector3};
use storm::graphics::*;
use storm::fontdue::layout::LayoutSettings;
use storm::fontdue::Font;
use storm::graphics::shaders::text::{Text, TextShader, TextShaderPass};
use super::*;

use crate::*;


//Reusable function that loads the character sprite, the shader to render it, and where on the screen it will be
pub fn load_character_sprite(animation_library: &AnimationTextureLibrary, character: &mut Character) -> ([Sprite; 1], SpriteShaderPass) { 
    let mut transform = Transform::new(window_logical_size());
    let mut sprite_1 = SpriteShaderPass::new(transform.matrix());

    sprite_1.atlas = animation_library.get_atlas_for_animation(character.animation_state);
    //And set the texture of the sprite as the subsection of the atlas for the first frame of animation
    let frame_1 = character.get_current_animation_config();
    let frame_1 = animation_library.get_atlas_subsection(character.animation_state, frame_1.current_frame);

    let sprites_1 = [
        Sprite {
            pos: Vector3::new(0.0, -(FRAME_HEIGHT as f32) * 3.0, 0.0),
            size: Vector2::new(FRAME_WIDTH as u16 * X_SCALE, FRAME_HEIGHT as u16 * Y_SCALE),
            color: RGBA8::WHITE,
            texture: frame_1,
            ..Default::default()
        }
    ];
    sprite_1.buffer.set(&sprites_1);

    return (sprites_1, sprite_1);
}

//Load the background, this is a bad function, redo it
pub fn setup_background() -> ([Sprite; 1], SpriteShaderPass) {
    let mut transform = Transform::new(window_logical_size());
    let mut background_sprite_pass = SpriteShaderPass::new(transform.matrix());
    let loaded_texture = Texture::from_png(BACKGROUND_CASTLE);
    let first_frame = loaded_texture.subsection(112, 787, 256, 512);
    background_sprite_pass.atlas = loaded_texture;
    let background_sprite = [
        Sprite {
            pos: Vector3::new(-1280.0, -500.0, -0.1),
            size: Vector2::new(896 * 3, 512 * 3),
            color: RGBA8::WHITE,
            texture: first_frame,
            ..Default::default()
        }
    ];

    return (background_sprite, background_sprite_pass);
}

pub fn setup_ui_backplate() -> ([Sprite; 1], SpriteShaderPass) {
    let mut transform = Transform::new(window_logical_size());
    let mut background_sprite_pass = SpriteShaderPass::new(transform.matrix());
    let loaded_texture = Texture::from_png(UI_BACKPLATE);
    let first_frame = loaded_texture.subsection(0, loaded_texture.width(), 0, loaded_texture.height());


    let background_sprite = [
        Sprite {
            pos: Vector3::new(-1500.0, 200.0, -0.09),
            size: Vector2::new(loaded_texture.width() as u16 * 3, (loaded_texture.height() as u16 * 3) / 2),
            color: RGBA8::WHITE,
            texture: first_frame,
            ..Default::default()
        }
    ];
    background_sprite_pass.atlas = loaded_texture;
    return (background_sprite, background_sprite_pass);
}

pub fn setup_join_game_button() -> ([Sprite; 1], SpriteShaderPass) {
    let mut transform = Transform::new(window_logical_size());
    let mut background_sprite_pass = SpriteShaderPass::new(transform.matrix());
    let loaded_texture = Texture::from_png(BUTTON);
    let first_frame = loaded_texture.subsection(0, loaded_texture.width(), 0, loaded_texture.height());


    let background_sprite = [
        Sprite {
            pos: Vector3::new(-1500.0, 200.0, -0.09),
            size: Vector2::new(loaded_texture.width() as u16 * 3, (loaded_texture.height() as u16 * 3) / 2),
            color: RGBA8::WHITE,
            texture: first_frame,
            ..Default::default()
        }
    ];
    background_sprite_pass.atlas = loaded_texture;
    return (background_sprite, background_sprite_pass);
}


pub struct UI {
    pub backplate: ([Sprite; 1], SpriteShaderPass),
    pub healthbars: ([Sprite; 2], SpriteShaderPass),
    pub timer_text: (TextShaderPass, TextShader)
}


pub fn setup_health_bars() -> ([Sprite; 2], SpriteShaderPass){
    let mut transform = Transform::new(window_logical_size());
    let mut health_bar_render_pass = SpriteShaderPass::new(transform.matrix());
    let loaded_texture = Texture::from_png(HEALTH_BAR_GRADIANT);
    let first_frame = loaded_texture.subsection(0, loaded_texture.width(), 0, loaded_texture.height());
    let health_bars = [
        Sprite {
            pos: Vector3::new(193.0, 364.0, 0.0),
            size: Vector2::new(319 * 5, 23 * 3 / 2),
            color: RGBA8::WHITE,
            texture: first_frame,
            ..Default::default()
        },
        Sprite {
            pos: Vector3::new(-193.0 - 934.0, 364.0, 0.0),
            size: Vector2::new(319 * 3, 23 * 3 / 2),
            color: RGBA8::WHITE,
            texture: first_frame,
            ..Default::default()
        },
    ];
    health_bar_render_pass.atlas = loaded_texture;
    health_bar_render_pass.buffer.set(&health_bars);
    return (health_bars, health_bar_render_pass);
}

//Load the sprites for te health bars, and there shader pass
pub fn setup_ui() -> UI {
    UI {
        backplate: setup_ui_backplate(),
        healthbars: setup_health_bars(),
        timer_text: setup_round_timer_text()
    }
}

//Load the sprites for te health bars, and there shader pass
pub fn setup_fireball() -> ([Sprite; 1], SpriteShaderPass){
    let mut transform = Transform::new(window_logical_size());
    let mut fireball_render_pass = SpriteShaderPass::new(transform.matrix());
    let character_y = -(FRAME_HEIGHT as f32) * 0.75;   
    let fireball_sprites = [
        Sprite {
            pos: Vector3::new(0.0, character_y * Y_SCALE as f32, 0.0),
            size: Vector2::new(FRAME_WIDTH as u16 * 2, FRAME_HEIGHT as u16 * 2),
            color: RGBA8::WHITE,
            ..Default::default()
        }
    ];
    fireball_render_pass.buffer.set(&fireball_sprites);
    return (fireball_sprites, fireball_render_pass);
}

//Load the sprites and the text shader pass used for the timer
pub fn setup_round_timer_text() -> (TextShaderPass, TextShader) {
    let mut transform = Transform::new(window_logical_size());

    let text_shader = TextShader::new();

    // Create a Layers to draw on.
    let mut text_layer = TextShaderPass::new(transform.matrix());

    // Setup the layout for our text.
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: -120.0,
        y: -130.0,
        max_width: Some(500.0),
        ..Default::default()
    };
    text_layer.set_ortho(transform.matrix());
    text_layer.append(
        &fonts,
        &layout_settings,
        &[Text {
            text: &String::from("60"),
            font_index: 0,
            px: 100.0,
            color: RGBA8::BLACK,
            depth: 0.0,
        }],
    );

    return (text_layer, text_shader);
}

/*

//Load the sprites and the text shader pass used for the timer
pub fn setup_round_reset_timer_text() -> (TextShaderPass, TextShader) {
    let mut transform = Transform::new(window_logical_size());

    let text_shader = TextShader::new();

    // Create a Layers to draw on.
    let mut text_layer = TextShaderPass::new(transform.matrix());

    // Setup the layout for our text.
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: 0.0,
        y: -200.0,
        max_width: Some(500.0),
        ..Default::default()
    };
    text_layer.set_ortho(transform.matrix());
    text_layer.append(
        &fonts,
        &layout_settings,
        &[Text {
            text: &String::from("60"),
            font_index: 0,
            px: 200.0,
            color: RGBA8::BLACK,
            depth: 0.0,
        }],
    );

    return (text_layer, text_shader);
}
 */