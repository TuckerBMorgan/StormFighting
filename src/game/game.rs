
use core::time::Duration;

extern crate simplelog;


use storm::color::RGBA8;
use storm::cgmath::{Vector3};
use storm::graphics::*;
use storm::event::*;
use storm::graphics::Texture;

use instant::{Instant};
use storm::fontdue::layout::LayoutSettings;
use storm::fontdue::Font;
use storm::graphics::shaders::text::{Text};
use ggrs::{GGRSError, SessionState, P2PSession};

use hashbrown::HashMap;

use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell, PlayerHandle, NULL_FRAME};

use super::*;
use super::character::AnimationStateForCharacterState;

pub const CHECKSUM_PERIOD: i32 = 100;

use crate::*;

/// computes the fletcher16 checksum, copied from wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for index in 0..data.len() {
        sum1 = (sum1 + data[index] as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}

pub struct GameConfig {
    pub collision_library: CollisionLibrary,
    pub combo_library: ComboLibrary,
    pub animation_library: AnimationTextureLibrary,
    pub animation_for_character_state_library: HashMap<CharacterState, AnimationStateForCharacterState>,
    pub animation_configs: HashMap<AnimationState, AnimationConfig>
}

impl GameConfig {
    pub fn new(collision_library: CollisionLibrary, 
               combo_library: ComboLibrary,
               animation_library: AnimationTextureLibrary,
               animation_for_character_state_library: HashMap<CharacterState, AnimationStateForCharacterState>,
               animation_configs: HashMap<AnimationState, AnimationConfig>) -> GameConfig {
        GameConfig {
            collision_library,
            combo_library,
            animation_library,
            animation_for_character_state_library,
            animation_configs
        }
    }
}

pub struct Game {
    pub current_round: Round,
    pub local_input: Input,
    pub last_checksum: (Frame, u64),
    pub periodic_checksum: (Frame, u64),
    pub game_config: GameConfig,
    pub p2p_session: P2PSession,
    pub local_handle: usize,
    pub ui: UI,
    pub character_1_sprites: [Sprite;1],
    pub sprite_pass_1: SpriteShaderPass,
    pub character_2_sprites: [Sprite;1],
    pub sprite_pass_2: SpriteShaderPass,
    pub sprite_shader: SpriteShader,
    pub projectile_sprites: Vec<([Sprite;1], SpriteShaderPass)>,
    pub fonts: [Font;1],
    pub fireball_texture: Texture,
    pub last_update: Instant,
    pub accumulator: Duration
}

impl Game {
    
    pub fn key_down(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_down(keyboard_button);
    }

    pub fn key_up(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_up(keyboard_button);
    }

    // deserialize gamestate to load and overwrite current gamestate
    pub fn load_game_state(&mut self, cell: GameStateCell) {
        let state_to_load = cell.load();
        self.current_round = bincode::deserialize(&state_to_load.buffer.unwrap()).unwrap();
    }

    // serialize current gamestate, create a checksum
    // creating a checksum here is only relevant for SyncTestSessions
    fn save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        // assert_eq!(self.game_state.frame, frame);
        let buffer = bincode::serialize(&self.current_round).unwrap();
        let checksum = fletcher16(&buffer) as u64;

        cell.save(GameState::new(frame, Some(buffer), Some(checksum)));
    }
    
    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        // advance the game state
        self.current_round.advance(inputs, &mut self.game_config);
        if self.current_round.round_done && self.current_round.reset_round_timer.finished() {
            self.current_round = Round::default();
        }

        // remember checksum to render it later
        // it is very inefficient to serialize the gamestate here just for the checksum
        let buffer = bincode::serialize(&self.current_round).unwrap();
        let checksum = fletcher16(&buffer) as u64;
        self.last_checksum = (self.current_round.frame, checksum);
        if self.current_round.frame % CHECKSUM_PERIOD == 0 {
            self.periodic_checksum = (self.current_round.frame, checksum);
        }
    }

    pub fn update(&mut self) {
        

        clear(ClearMode::color_depth(RGBA8::BLACK));
        self.p2p_session.poll_remote_clients();
        if self.p2p_session.current_state() == SessionState::Running {
            // this is to keep ticks between clients synchronized.
            // if a client is ahead, it will run frames slightly slower to allow catching up
            let mut fps_delta = 1. / FPS;
            if self.p2p_session.frames_ahead() > 0 {
                fps_delta *= 1.1;
            }

            // get delta time from last iteration and accumulate it
            let delta = Instant::now().duration_since(self.last_update);
            self.accumulator = self.accumulator.saturating_add(delta);
            self.last_update = Instant::now();

            // if enough time is accumulated, we run a frame
            while self.accumulator.as_secs_f64() > fps_delta {
                // decrease accumulator
                self.accumulator = self.accumulator.saturating_sub(Duration::from_secs_f64(fps_delta));

            
                match self.p2p_session.advance_frame(self.local_handle, &self.local_input(0)) {
                    Ok(requests) => self.handle_requests(requests),
                    Err(GGRSError::PredictionThreshold) => println!("Frame skipped"),
                    Err(e) => panic!("{:?}", e),
                }

                //TODO: maybe use a is_dirty flag to update this only when we need to
                self.sprite_pass_1.atlas = self.game_config.animation_library.get_atlas_for_animation(self.current_round.characters[0].animation_state);
                let frame = self.current_round.characters[0].get_current_animation_config();
                self.character_1_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[0].animation_state, frame.current_frame);
                self.character_1_sprites[0].pos.x = self.current_round.characters[0].character_position.x * X_SCALE as f32;

                self.sprite_pass_2.atlas = self.game_config.animation_library.get_atlas_for_animation(self.current_round.characters[1].animation_state);
                let frame = self.current_round.characters[1].get_current_animation_config();
                self.character_2_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[1].animation_state, frame.current_frame).mirror_y();
                self.character_2_sprites[0].pos.x = self.current_round.characters[1].character_position.x * X_SCALE as f32;

                if self.current_round.projectiles.len() != self.projectile_sprites.len() {
                    let diff = self.current_round.projectiles.len().abs_diff(self.projectile_sprites.len());
                    if self.current_round.projectiles.len() > self.projectile_sprites.len() {
                        //we need to add the number of new sprites
                        for _ in 0..diff {
                            let (fireball_sprite, mut fireball_render_pass) = setup_fireball();
                            fireball_render_pass.atlas = self.fireball_texture.clone();
                            self.projectile_sprites.push((fireball_sprite, fireball_render_pass))
                        }
                    }
                    else {
                        //remove the 
                        self.projectile_sprites.truncate(self.projectile_sprites.len() - diff);
                    }
                }

                for (index, projectile) in self.current_round.projectiles.iter().enumerate() {
                    match projectile.screen_side {
                        ScreenSide::Left => {
                            let left = projectile.timer.current_frame * FRAME_WIDTH;
                            let test = self.projectile_sprites[index].1.atlas.subsection(left, 0 + FRAME_WIDTH, 0, FRAME_HEIGHT).mirror_y();
                            self.projectile_sprites[index].0[0].texture = test;
                            self.projectile_sprites[index].0[0].pos = Vector3::new(projectile.position.x * X_SCALE as f32 - FRAME_WIDTH as f32 * X_SCALE as f32, projectile.position.y * Y_SCALE as f32, 0.0);
                        },
                        ScreenSide::Right => {
                            let left = projectile.timer.current_frame * FRAME_WIDTH;
                            let test = self.projectile_sprites[index].1.atlas.subsection(left, 0 + FRAME_WIDTH, 0, FRAME_HEIGHT);
                            self.projectile_sprites[index].0[0].texture = test;
                            self.projectile_sprites[index].0[0].pos = Vector3::new(projectile.position.x * X_SCALE as f32 , projectile.position.y * Y_SCALE as f32, 0.0);
                        }
                    }
                }
            }

            let text_color;
            let current_frame_count = 60 - (self.current_round.round_timer.current_frame / 60);
            if  current_frame_count > 20 {
                text_color = RGBA8::new(85, 196, 59, 255);
            } else if current_frame_count > 10 {
                text_color = RGBA8::YELLOW;
            }
            else {
                text_color = RGBA8::RED;
            }

            self.ui.timer_text.0.clear_text();

            let layout_settings = LayoutSettings {
                x: -30.0,
                y: 420.0,
                max_width: Some(200.0),
                ..Default::default()
            };

            self.ui.timer_text.0.append(
                &self.fonts,
                &layout_settings,
                &[Text {
                    text: &(60 - (self.current_round.round_timer.current_frame / 60)).to_string(),
                    font_index: 0,
                    px: 75.0,
                    color: text_color,
                    depth: 0.0,
                }],
            );

            self.ui.timer_text.0.draw(&self.ui.timer_text.1);        

            //Commit the current images to the screen
            //background_sprite_pass.buffer.set(&mut background_sprite);
            //background_sprite_pass.draw(&sprite_shader);

            self.ui.backplate.1.buffer.set(&mut self.ui.backplate.0);
            self.ui.backplate.1.draw(&self.sprite_shader);

            for projectile_sprites in self.projectile_sprites.iter_mut() {
                projectile_sprites.1.buffer.set(&projectile_sprites.0);
                projectile_sprites.1.draw(&self.sprite_shader);
            }

            self.sprite_pass_1.buffer.set(&self.character_1_sprites);
            self.sprite_pass_1.draw(&self.sprite_shader);
            
            self.sprite_pass_2.buffer.set(&self.character_2_sprites);
            self.sprite_pass_2.draw(&self.sprite_shader);
            self.ui.healthbars.0[0].size.x = ((319.0 * 3.0) * (self.current_round.characters[0].health as f32 / 250.0)) as u16;
            self.ui.healthbars.0[1].size.x = ((319.0 * 3.0) * (self.current_round.characters[1].health as f32 / 250.0)) as u16;
            self.ui.healthbars.0[1].pos.x = -193.0 - 934.0 + (((319.0 * 3.0) * ( 1.0 - (self.current_round.characters[1].health as f32 / 250.0))) as u16) as f32;

            self.ui.healthbars.1.buffer.set(&self.ui.healthbars.0);
            self.ui.healthbars.1.draw(&self.sprite_shader);
        }
    }

    // for each request, call the appropriate function
    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest>) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell, frame: _ } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    #[allow(dead_code)]
    // creates a compact representation of currently pressed keys and serializes it
    pub fn local_input(&self, handle: PlayerHandle) -> Vec<u8> {
        let mut input: u16 = 0;

        // ugly, but it works...
        // player 1 with WASD
        if handle == 0 {
            if self.local_input.left_key_down {
                input |= INPUT_LEFT;
            }
            if self.local_input.right_key_down {
                input |= INPUT_RIGHT;
            }
            if self.local_input.down_key_down {
                input |= INPUT_DOWN;
            }
            if self.local_input.light_attack {
                input |= INPUT_LIGHT_ATTACK;
            }
            if self.local_input.medium_attack {
                input |= INPUT_MEDIUM_ATTACK;
            }
            if self.local_input.heavy_attack {
                input |= INPUT_HEAVY_ATTACK;
            }
            if self.local_input.light_kick {
                input |= INPUT_LIGHT_KICK;
            }
            if self.local_input.medium_kick {
                input |= INPUT_MEDIUM_KICK;
            }
            if self.local_input.heavy_kick {
                input |= INPUT_HEAVY_KICK;
            }
        }
        return input.to_le_bytes().to_vec();
    }
}

impl Default for Game {
    fn default() -> Game {

        let mut animation_for_character_state_library = HashMap::new();
        animation_for_character_state_library.insert(CharacterState::Idle, AnimationStateForCharacterState::new(AnimationState::Crouched, AnimationState::Idle));
        animation_for_character_state_library.insert(CharacterState::ForwardRun, AnimationStateForCharacterState::new(AnimationState::ForwardRun, AnimationState::ForwardRun));
        animation_for_character_state_library.insert(CharacterState::BackwardRun, AnimationStateForCharacterState::new(AnimationState::BackwardRun, AnimationState::BackwardRun));
        animation_for_character_state_library.insert(CharacterState::LightHitRecovery, AnimationStateForCharacterState::new(AnimationState::LightHitRecovery, AnimationState::LightHitRecovery));
        animation_for_character_state_library.insert(CharacterState::Blocking, AnimationStateForCharacterState::new(AnimationState::Blocking, AnimationState::Blocking));
        animation_for_character_state_library.insert(CharacterState::Crouching, AnimationStateForCharacterState::new(AnimationState::Crouching, AnimationState::Crouching));
        animation_for_character_state_library.insert(CharacterState::LightAttack, AnimationStateForCharacterState::new(AnimationState::LightCrouchAttack, AnimationState::LightAttack));
        animation_for_character_state_library.insert(CharacterState::MediumAttack, AnimationStateForCharacterState::new(AnimationState::LightCrouchAttack, AnimationState::MediumAttack));
        animation_for_character_state_library.insert(CharacterState::HeavyAttack, AnimationStateForCharacterState::new(AnimationState::HeavyCrouchingAttack, AnimationState::HeavyAttack));
        animation_for_character_state_library.insert(CharacterState::LightKick, AnimationStateForCharacterState::new(AnimationState::LightKick, AnimationState::LightKick));
        animation_for_character_state_library.insert(CharacterState::MediumKick, AnimationStateForCharacterState::new(AnimationState::MediumKick, AnimationState::MediumKick));
        animation_for_character_state_library.insert(CharacterState::HeavyKick, AnimationStateForCharacterState::new(AnimationState::HeavyKick, AnimationState::HeavyKick));
        animation_for_character_state_library.insert(CharacterState::ForwardDash, AnimationStateForCharacterState::new(AnimationState::ForwardDash, AnimationState::ForwardDash));
        animation_for_character_state_library.insert(CharacterState::BackwardDash, AnimationStateForCharacterState::new(AnimationState::BackwardDash, AnimationState::BackwardDash));
        animation_for_character_state_library.insert(CharacterState::Special1, AnimationStateForCharacterState::new(AnimationState::Special1, AnimationState::Special1));
        animation_for_character_state_library.insert(CharacterState::Won, AnimationStateForCharacterState::new(AnimationState::Won, AnimationState::Won));
        animation_for_character_state_library.insert(CharacterState::Lost, AnimationStateForCharacterState::new(AnimationState::Lost, AnimationState::Lost));


        let mut animation_configs = HashMap::new();
        animation_configs.insert(AnimationState::Idle,                 AnimationConfig::new(10, 4));
        animation_configs.insert(AnimationState::ForwardRun,           AnimationConfig::new(12, 4));
        animation_configs.insert(AnimationState::BackwardRun,          AnimationConfig::new(10, 4));
        animation_configs.insert(AnimationState::LightAttack,          AnimationConfig::new(5, 4));
        animation_configs.insert(AnimationState::MediumAttack,         AnimationConfig::new(8, 4));
        animation_configs.insert(AnimationState::HeavyAttack,          AnimationConfig::new(11, 4));
        animation_configs.insert(AnimationState::LightHitRecovery,     AnimationConfig::new(4, 4));
        animation_configs.insert(AnimationState::Blocking,             AnimationConfig::new(4, 4));
        animation_configs.insert(AnimationState::Crouched,             AnimationConfig::new(4, 4));
        animation_configs.insert(AnimationState::Crouching,            AnimationConfig::new(2, 4));
        animation_configs.insert(AnimationState::LightCrouchAttack,    AnimationConfig::new(5, 4));
        animation_configs.insert(AnimationState::HeavyCrouchingAttack, AnimationConfig::new(9, 4));
        animation_configs.insert(AnimationState::LightKick,            AnimationConfig::new(6, 4));
        animation_configs.insert(AnimationState::MediumKick,           AnimationConfig::new(8, 4));
        animation_configs.insert(AnimationState::HeavyKick,            AnimationConfig::new(13, 4));
        animation_configs.insert(AnimationState::ForwardDash,          AnimationConfig::new(6, 4));
        animation_configs.insert(AnimationState::BackwardDash,         AnimationConfig::new(6, 4));
        animation_configs.insert(AnimationState::Special1,             AnimationConfig::new(14, 4));
        animation_configs.insert(AnimationState::Won,                  AnimationConfig::new(6, 5));
        animation_configs.insert(AnimationState::Lost,                 AnimationConfig::new(5, 5));




        let mut current_round = Round::default();
        let (p2p_session, local_handle) = launch_session();
        let (mut background_sprite, mut background_sprite_pass) = setup_background();
        clear(ClearMode::color_depth(RGBA8::BLACK));
        let sprite_shader = SpriteShader::new();


    
    
        //Load a sprite with the atlas of whatever the idle animation is
    
        let animation_library = AnimationTextureLibrary::default();
        //Load the characters sprites and shaders
    
        let (sprites_1, 
            sprite_pass_1) = load_character_sprite(&animation_library, &mut current_round.characters[0]);
        let (sprites_2, 
            sprite_pass_2) = load_character_sprite(&animation_library, &mut current_round.characters[1]);
    
        let fireball_texture = Texture::from_png(FIREBALL);

    
        let ui = setup_ui();
    
        //load the font used for the timer
        let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];



        let game_config = GameConfig::new(CollisionLibrary::default(), ComboLibrary::default(), animation_library, animation_for_character_state_library, animation_configs);
        let last_update = Instant::now();
        let accumulator = Duration::ZERO;
        Game {
            current_round,
            local_input: Input::new(),
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            game_config,
            p2p_session,
            local_handle,
            ui,
            character_1_sprites: sprites_1,
            sprite_pass_1,            
            character_2_sprites: sprites_2,
            sprite_pass_2,
            sprite_shader,
            projectile_sprites: vec![],
            fonts,
            fireball_texture,
            last_update,
            accumulator
        }
    }
}