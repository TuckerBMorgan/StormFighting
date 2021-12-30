use core::convert::{From};
use core::time::Duration;

extern crate simplelog;
use simplelog::*;

use storm::color::RGBA8;
use storm::cgmath::{Vector2, Vector3};
use storm::*;
use storm::event::*;
use storm::graphics::Texture;
use storm::math::Transform;
use instant::{Instant};
use storm::fontdue::layout::LayoutSettings;
use storm::fontdue::Font;
use storm::graphics::shaders::text::{Text, TextShader, TextShaderPass};
use ggrs::{GGRSError, SessionState};

mod game;
mod shaders;

use game::*;
use shaders::*;

static FONT: &[u8] = include_bytes!("resources/ka1.ttf");


const X_SCALE : u16 = 4;
const Y_SCALE : u16 = 4;
#[derive(Eq, PartialEq)]
pub enum AppState {
    Play,
    Pause,
    Debug
}

fn main() {
    //I am initing this logger to avoid an error on mac
    let _ = SimpleLogger::init(LevelFilter::Warn, Config::default());
    // Create the engine context and describe the window.
    Context::start(
        WindowSettings {
            title: String::from("Storm Fighting"),
            display_mode: DisplayMode::Windowed {
                width: 1280 * 2,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
        },
        run,
    );
}


//Reusable function that loads the character sprite, the shader to render it, and where on the screen it will be
fn load_character_sprite(animation_library: &AnimationTextureLibrary, ctx: &Context, character: &mut Character) -> ([Sprite; 1], SpriteShaderPass) { 
    let mut transform = Transform::new(ctx.window_logical_size());
    let mut sprite_1 = SpriteShaderPass::new(transform.matrix());

    sprite_1.atlas = animation_library.get_atlas_for_animation(character.animation_state);
    //And set the texture of the sprite as the subsection of the atlas for the first frame of animation
    let frame_1 = character.get_current_animation_config();
    let frame_1 = animation_library.get_atlas_subsection(character.animation_state, frame_1.current_frame);
    let character_y = -(FRAME_HEIGHT as f32) * 0.75;    
    let sprites_1 = [
        Sprite {
            pos: Vector3::new(character.character_position.x * X_SCALE as f32, character_y * Y_SCALE as f32, 0.0),
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
fn setup_background(ctx: &Context) -> ([Sprite; 1], SpriteShaderPass) {
    let mut transform = Transform::new(ctx.window_logical_size());
    let mut background_sprite_pass = SpriteShaderPass::new(transform.matrix());
    let loaded_texture = Texture::from_png(BACKGROUND_CASTLE);
    let first_frame = loaded_texture.subsection(0, 896, 0, 512);
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

//Load the sprites for te health bars, and there shader pass
fn setup_healthbars(ctx: &Context) -> ([Sprite; 2], SpriteShaderPass){
    let mut transform = Transform::new(ctx.window_logical_size());
    let mut health_bar_render_pass = SpriteShaderPass::new(transform.matrix());

    let health_bars = [
        Sprite {
            pos: Vector3::new(100.0, 400.0, 0.0),
            size: Vector2::new(750, 50),
            color: RGBA8::WHITE,
            ..Default::default()
        },
        Sprite {
            pos: Vector3::new(-900.0, 400.0, 0.0),
            size: Vector2::new(750, 50),
            color: RGBA8::WHITE,
            ..Default::default()
        },
    ];
    health_bar_render_pass.buffer.set(&health_bars);
    return (health_bars, health_bar_render_pass);
}

//Load the sprites and the text shader pass used for the timer
fn setup_round_timer_text(ctx: &Context) -> (TextShaderPass, TextShader) {
    let mut transform = Transform::new(ctx.window_logical_size());

    let text_shader = TextShader::new();

    // Create a Layers to draw on.
    let mut text_layer = TextShaderPass::new(transform.matrix());

    // Setup the layout for our text.
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: -120.0,
        y: 500.0,
        max_width: Some(500.0),
        ..Default::default()
    };
    text_layer.set_ortho(transform.generate());
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

fn run(ctx: &mut Context) -> impl FnMut(Event, &mut Context) {

    let (mut p2p_session, local_handle) = launch_session();


    let mut app_state = AppState::Play;

    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 60.0)));

    let mut game = Game::default();
    //Load a sprite with the atlas of whatever the idle animation is

    let animation_library = AnimationTextureLibrary::default();

    //Load the characters sprites and shaders
    let sprite_shader = SpriteShader::new();
    let (mut sprites_1, 
        mut sprite_1) = load_character_sprite(&animation_library, ctx, &mut game.current_round.character_1);
    let (mut sprites_2, 
        mut sprite_2) = load_character_sprite(&animation_library, ctx,&mut game.current_round.character_2);

    let (mut background_sprite, mut background_sprite_pass) = setup_background(ctx);
    let (mut health_bars, mut health_bar_render_pass)  = setup_healthbars(ctx);
    let (mut text_layer, text_shader) = setup_round_timer_text(ctx);

    //load the font used for the timer
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: -120.0,
        y: 500.0,
        max_width: Some(500.0),
        ..Default::default()
    };


    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;
    move |event, ctx| match event {
        //Process input
        //A subset of these keys go to the character with character_1.key_down or character_1.key_up
        //Others are used to control game flow, like Pause, or Debug
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => ctx.stop(),
            _ => {
                game.key_down(key);
            }
        },
        Event::KeyReleased(key) => match key {
            KeyboardButton::P => {
                if app_state == AppState::Play {
                    app_state = AppState::Pause;
                }
                else {
                    app_state = AppState::Play;
                }
            }
            KeyboardButton::O => {
                app_state = AppState::Debug;
            }
            KeyboardButton::Escape => ctx.stop(),
            _ => {
                game.key_up(key);
            }
        },
        
        Event::Update(_delta) => {
            clear(ClearMode::color_depth(RGBA8::BLACK));
            p2p_session.poll_remote_clients();
            if p2p_session.current_state() == SessionState::Running {
                // this is to keep ticks between clients synchronized.
                // if a client is ahead, it will run frames slightly slower to allow catching up
                let mut fps_delta = 1. / FPS;
                if p2p_session.frames_ahead() > 0 {
                    fps_delta *= 1.1;
                }
    
                // get delta time from last iteration and accumulate it
                let delta = Instant::now().duration_since(last_update);
                accumulator = accumulator.saturating_add(delta);
                last_update = Instant::now();
    
                // if enough time is accumulated, we run a frame
                while accumulator.as_secs_f64() > fps_delta {
                    // decrease accumulator
                    accumulator = accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

                
                    match p2p_session.advance_frame(local_handle, &game.local_input(0)) {
                        Ok(requests) => game.handle_requests(requests),
                        Err(GGRSError::PredictionThreshold) => println!("Frame skipped"),
                        Err(e) => panic!("{:?}", e),
                    }

                    //TODO: maybe use a is_dirty flag to update this only when we need to
                    sprite_1.atlas = animation_library.get_atlas_for_animation(game.current_round.character_1.animation_state);
                    let frame = game.current_round.character_1.get_current_animation_config();
                    sprites_1[0].texture = animation_library.get_atlas_subsection(game.current_round.character_1.animation_state, frame.current_frame);
                    sprites_1[0].pos.x = game.current_round.character_1.character_position.x * X_SCALE as f32;

                    sprite_2.atlas = animation_library.get_atlas_for_animation(game.current_round.character_2.animation_state);
                    let frame = game.current_round.character_2.get_current_animation_config();
                    sprites_2[0].texture = animation_library.get_atlas_subsection(game.current_round.character_2.animation_state, frame.current_frame).mirror_y();
                    sprites_2[0].pos.x = game.current_round.character_2.character_position.x * X_SCALE as f32;

                }
                
                text_layer.clear_text();
                text_layer.append(
                    &fonts,
                    &layout_settings,
                    &[Text {
                        text: &(60 - (game.current_round.round_timer.current_frame / 60)).to_string(),
                        font_index: 0,
                        px: 100.0,
                        color: RGBA8::BLACK,
                        depth: 0.0,
                    }],
                );

                text_layer.draw(&text_shader);
                
                //Commit the current images to the screen
                background_sprite_pass.buffer.set(&mut background_sprite);
                background_sprite_pass.draw(&sprite_shader);

                sprite_1.buffer.set(&sprites_1);
                sprite_1.draw(&sprite_shader);

                sprite_2.buffer.set(&sprites_2);
                sprite_2.draw(&sprite_shader);
                health_bars[0].size.x = (750.0 * (game.current_round.character_1.health as f32 / 250.0)) as u16;
                health_bars[1].size.x = (750.0 * (game.current_round.character_2.health as f32 / 250.0)) as u16;

                health_bar_render_pass.buffer.set(&health_bars);
                health_bar_render_pass.draw(&sprite_shader);
            }
        }
        _ => {}
    }
}
