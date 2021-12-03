use core::convert::{From};
use core::time::Duration;

use std::net::SocketAddr;

use storm::cgmath::{Vector2, Vector3};
use storm::*;

use instant::{Instant};

use structopt::StructOpt;
use ggrs::{GGRSError, P2PSession, PlayerType, SessionState};

mod game;

use game::*;

const FPS: f64 = 60.0;
const INPUT_SIZE: usize = std::mem::size_of::<u8>();

const X_SCALE : u16 = 4;
const Y_SCALE : u16 = 4;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

#[derive(Eq, PartialEq)]
pub enum AppState {
    Play,
    Pause,
    Debug
}

fn main() {
    // Create the engine context and describe the window.
    Context::start(
        WindowSettings {
            title: String::from("Storm Fighting"),
            display_mode: DisplayMode::Windowed {
                width: 1280 * 2,
                height: 1024,
                resizable: false,
            },
            vsync: Vsync::Disabled,
        },
        run,
    );
}

fn launch_session() -> (P2PSession, usize) {
    // read cmd line arguments
    let opt = Opt::from_args();
    let mut local_handle = 0;
    let num_players = opt.players.len();
    assert!(num_players > 0);

    // create a GGRS session
    let mut sess = P2PSession::new(num_players as u32, INPUT_SIZE, opt.local_port).unwrap();

    // turn on sparse saving
    sess.set_sparse_saving(true).unwrap();

    // set FPS (default is 60, so this doesn't change anything as is)
    sess.set_fps(FPS as u32).unwrap();

    // add players
    for (i, player_addr) in opt.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            sess.add_player(PlayerType::Local, i).unwrap();
            local_handle = i;
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse().unwrap();
            sess.add_player(PlayerType::Remote(remote_addr), i).unwrap();
        }
    }

    // optionally, add spectators
    for (i, spec_addr) in opt.spectators.iter().enumerate() {
        sess.add_player(PlayerType::Spectator(*spec_addr), num_players + i).unwrap();
    }

    // set input delay for the local player
    sess.set_frame_delay(4, local_handle).unwrap();

    // set change default expected update frequency
    sess.set_fps(FPS as u32).unwrap();

    // start the GGRS session
    sess.start_session().unwrap();
    return (sess, local_handle);
}


fn run(ctx: &mut Context) -> impl FnMut(Event, &mut Context) {

    let (mut p2p_session, local_handle) = launch_session();


    let mut app_state = AppState::Play;

    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 60.0)));

    let mut animation_library = AnimationTextureLibrary::new();
    animation_library.load_animation(ctx, IDLE_TEXTURE, AnimationState::Idle);
    animation_library.load_animation(ctx, FORWARD_RUN_TEXTURE, AnimationState::ForwardRun);
    animation_library.load_animation(ctx, BACKGROUND_RUN_TEXTURE, AnimationState::BackwardRun);
    animation_library.load_animation(ctx, LIGHT_ATTACK_TEXTURE, AnimationState::LightAttack);

    let mut game = Game::default();
    //Load a sprite with the atlas of whatever the idle animation is
    let mut sprite_1 = ctx.sprite_layer();
    sprite_1.set_atlas(animation_library.get_atlas_for_animation(game.current_round.character_1.animation_state));
    //And set the texture of the sprite as the subsection of the atlas for the first frame of animation
    let frame_1 = game.current_round.character_1.get_current_animation_config();
    let frame_1 = animation_library.get_atlas_subsection(game.current_round.character_1.animation_state, frame_1.current_frame);

    let character_y = -(FRAME_HEIGHT as f32) / 2.0;

    let first_character_x = -(FRAME_WIDTH as f32) / 2.0; 
    game.current_round.character_1.character_position.x  = first_character_x;
    let mut sprites_1 = [
        Sprite {
            pos: Vector3::new(first_character_x * X_SCALE as f32, character_y * Y_SCALE as f32, 0.0),
            size: Vector2::new(FRAME_WIDTH as u16 * X_SCALE, FRAME_HEIGHT as u16 * Y_SCALE),
            color: RGBA8::WHITE,
            texture: frame_1,
            ..Default::default()
        }
    ];
    sprite_1.set_sprites(&sprites_1);

    //Load a sprite with the atlas of whatever the idle animation is
    let mut sprite_2 = ctx.sprite_layer();
    sprite_2.set_atlas(animation_library.get_atlas_for_animation(game.current_round.character_2.animation_state));
    //And set the texture of the sprite as the subsection of the atlas for the first frame of animation
    let frame_1 = game.current_round.character_2.get_current_animation_config();
    let frame_1 = animation_library.get_atlas_subsection(game.current_round.character_2.animation_state, frame_1.current_frame);
    let second_character_x = -(FRAME_WIDTH as f32); 
    game.current_round.character_2.character_position.x  = second_character_x;

    let mut sprites_2 = [
        Sprite {
            pos: Vector3::new(second_character_x * X_SCALE as f32, character_y * Y_SCALE as f32, 0.0),
            size: Vector2::new(FRAME_WIDTH as u16 * X_SCALE, FRAME_HEIGHT as u16 * Y_SCALE),
            color: RGBA8::WHITE,
            texture: frame_1.mirror_y(),
            ..Default::default()
        }
    ];
    sprite_2.set_sprites(&sprites_2);

    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;
    move |event, ctx| match event {
        //Process input
        //A subset of these keys go to the character with character_1.key_down or character_1.key_up
        //Others are used to control game flow, like Pause, or Debug
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Left => {
                game.key_down(key);
            }
            KeyboardButton::Right => {
                game.key_down(key);
            }
            KeyboardButton::Q => game.key_down(key),
            KeyboardButton::Escape => ctx.stop(),
            _ => {}
        },
        Event::KeyReleased(key) => match key {

            KeyboardButton::Left => {
                game.key_up(key);
            }
            KeyboardButton::Right => {
                game.key_up(key);
            }
            KeyboardButton::Q => game.key_up(key),
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
            _ => {}
        },
        
        Event::Update(_delta) => {
            ctx.clear(ClearMode::color_depth(RGBA8::BLACK));
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
                    sprite_1.set_atlas(animation_library.get_atlas_for_animation(game.current_round.character_1.animation_state));
                    let frame = game.current_round.character_1.get_current_animation_config();
                    sprites_1[0].texture = animation_library.get_atlas_subsection(game.current_round.character_1.animation_state, frame.current_frame);
                    sprites_1[0].pos.x = game.current_round.character_1.character_position.x * X_SCALE as f32;

                    sprite_2.set_atlas(animation_library.get_atlas_for_animation(game.current_round.character_2.animation_state));
                    let frame = game.current_round.character_2.get_current_animation_config();
                    sprites_2[0].texture = animation_library.get_atlas_subsection(game.current_round.character_2.animation_state, frame.current_frame).mirror_y();
                    sprites_2[0].pos.x = game.current_round.character_2.character_position.x * X_SCALE as f32;
                }
                //Commit the current images to the screen
                sprite_1.set_sprites(&sprites_1);
                sprite_1.draw();

                sprite_2.set_sprites(&sprites_2);
                sprite_2.draw();
            }
        }
        _ => {}
    }
}
