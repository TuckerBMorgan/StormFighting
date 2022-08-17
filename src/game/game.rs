
use core::time::Duration;

extern crate simplelog;

use storm::cgmath::Vector3;
use storm::color::RGBA8;

use storm::graphics::*;
use storm::event::*;
use storm::graphics::Texture;

use instant::{Instant};
use storm::fontdue::layout::LayoutSettings;
use storm::fontdue::Font;
use storm::graphics::shaders::text::{Text};
use ggrs::{GGRSError};

use hashbrown::HashMap;

use ggrs::{Frame, GGRSRequest, GameStateCell, PlayerHandle, NULL_FRAME, InputStatus};
use storm::math::OrthographicCamera;

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
    pub animation_configs: HashMap<AnimationState, AnimationConfig>,
    pub character_sheet: CharacterSheet,
    pub pallete: [cgmath::Vector3<f32>; 256]
}

impl GameConfig {
    pub fn new(collision_library: CollisionLibrary, 
               combo_library: ComboLibrary,
               animation_library: AnimationTextureLibrary,
               animation_for_character_state_library: HashMap<CharacterState, AnimationStateForCharacterState>,
               animation_configs: HashMap<AnimationState, AnimationConfig>,
               character_sheet: CharacterSheet,
               pallete: [cgmath::Vector3<f32>; 256]) -> GameConfig {
        GameConfig {
            collision_library,
            combo_library,
            animation_library,
            animation_for_character_state_library,
            animation_configs,
            character_sheet,
            pallete
        }
    }
}

pub struct Game<'a> {
    pub current_round: Round,
    pub local_input: Input,
    pub last_checksum: (Frame, u64),
    pub periodic_checksum: (Frame, u64),
    pub game_config: GameConfig,
    pub net: Net<'a>,
    pub ui: UI,

    pub character_1_sprites: [PalleteSprite;1],
    pub sprite_pass_1: PalleteSpriteShaderPass,
    pub character_2_sprites: [PalleteSprite;1],
    pub sprite_pass_2: PalleteSpriteShaderPass,
    pub pallete_sprite_shader: PalleteSpriteShader,
    pub sprite_shader: SpriteShader,

    pub projectile_sprites: Vec<([Sprite;1], SpriteShaderPass)>,
    pub effects_sprites: Vec<([Sprite;1], SpriteShaderPass)>,
    pub fonts: [Font;1],
    pub fireball_texture: Texture,
    pub light_hit_effect_texture: Texture,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub background_sprite: [Sprite;1],
    pub background_sprite_pass: SpriteShaderPass,
    pub camera_transform: OrthographicCamera
}

impl<'a> Game<'a> {
    
    pub fn load_game_with_config(ctx: &mut Context<FighthingApp>, mut game_config: GameConfig) -> Game<'a> {

        
        let mut current_round = Round::new_with_animation_lib(&mut game_config.animation_configs);
        let net = Net::launch_session();

        let (background_sprite, background_sprite_pass) = setup_background(ctx);
        let sprite_shader = SpriteShader::new(ctx);
        let pallete_sprite_shader = PalleteSpriteShader::new(ctx);
        let (sprites_1, 
            sprite_pass_1) = load_character_sprite(&game_config.animation_library, &mut current_round.characters[0], ctx, game_config.pallete);
        let (sprites_2, 
            sprite_pass_2) = load_character_sprite(&game_config.animation_library, &mut current_round.characters[1], ctx, game_config.pallete);
        let fireball_texture = Texture::from_png(ctx, FIREBALL, TextureFiltering::none());
        let light_hit_effect_texture = Texture::from_png(ctx, LIGHT_HIT_EFFECT_TEXTURE, TextureFiltering::none());
        let ui = setup_ui(ctx);    
        //load the font used for the timer
        let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];

        let last_update = Instant::now();
        let accumulator = Duration::ZERO;
        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        
        //-108 is HEIGHT / 2 / SCALING FACTOR
        transform.set().translation = Vector3::new(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0 / 5.0, 0.0);
        transform.set().scale = 0.25;
        Game {
            current_round,
            local_input: Input::new(),
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            game_config,
            net,
            ui,
            character_1_sprites: sprites_1,
            sprite_pass_1,            
            character_2_sprites: sprites_2,
            sprite_pass_2,
            sprite_shader,
            pallete_sprite_shader,
            projectile_sprites: vec![],
            effects_sprites: vec![],
            fonts,
            fireball_texture,
            light_hit_effect_texture,
            last_update,
            accumulator,
            background_sprite,
            background_sprite_pass,
            camera_transform: transform
        }
    }


    pub fn key_down(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_down(keyboard_button);
    }

    pub fn key_up(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_up(keyboard_button);
    }

    // deserialize gamestate to load and overwrite current gamestate
    pub fn load_game_state(&mut self, cell: GameStateCell<Round>) {
        self.current_round = cell.load().expect("No data found.");
    }

    // serialize current gamestate, create a checksum
    // creating a checksum here is only relevant for SyncTestSessions
    fn save_game_state(&mut self, cell: GameStateCell<Round>, frame: Frame) {
        // assert_eq!(self.game_state.frame, frame);
        let buffer = bincode::serialize(&self.current_round).unwrap();
        let checksum = fletcher16(&buffer) as u128;

        cell.save(frame, Some(self.current_round.clone()), Some(checksum));
    }
    
    fn advance_frame(&mut self, inputs: Vec<(NetInput, InputStatus)>) {
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

    pub fn update(&mut self, ctx: &mut Context<FighthingApp>) {


        ctx.clear(ClearMode::new().with_color(RGBA8::BLUE).with_depth(0.0, DepthTest::Greater));
        
        self.net.tick();
        match self.net.state {
            NetState::Connecting => {
                return;
            },
            _ => {

            }
        }


        if self.net.is_running() {
            // this is to keep ticks between clients synchronized.
            // if a client is ahead, it will run frames slightly slower to allow catching up
            let mut fps_delta = 1. / FPS;
            if self.net.session.as_mut().unwrap().frames_ahead() > 0 {
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
                self.net.add_local_input(self.net.local_handle, self.local_input(self.net.local_handle));
                match self.net.session.as_mut().unwrap().advance_frame() {
                    Ok(requests) => self.handle_requests(requests),
                    Err(GGRSError::PredictionThreshold) => println!("Frame skipped"),
                    Err(e) => panic!("{:?}", e),
                }

                //Update all of the sprites positions
                //TODO: maybe use a is_dirty flag to update this only when we need to
                self.sprite_pass_1.atlas = self.game_config.animation_library.get_atlas_for_animation(self.current_round.characters[0].animation_state);
                
                let frame = self.current_round.characters[0].get_current_animation_config();
                if self.current_round.characters[0].screen_side == ScreenSide::Right {
                    self.character_1_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[0].animation_state, frame.current_frame);
                }
                else {
                    self.character_1_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[0].animation_state, frame.current_frame).mirror_y();
                }
                
                self.character_1_sprites[0].pos.x = self.current_round.characters[0].character_position.x;
                self.character_1_sprites[0].pos.y = self.current_round.characters[0].character_position.y;

                self.sprite_pass_2.atlas = self.game_config.animation_library.get_atlas_for_animation(self.current_round.characters[1].animation_state);
                let frame = self.current_round.characters[1].get_current_animation_config();
                if self.current_round.characters[0].screen_side == ScreenSide::Left {
                    self.character_2_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[1].animation_state, frame.current_frame);
                }
                else {
                    self.character_2_sprites[0].texture = self.game_config.animation_library.get_atlas_subsection(self.current_round.characters[1].animation_state, frame.current_frame).mirror_y();
                }

                self.character_2_sprites[0].pos.x = self.current_round.characters[1].character_position.x;
                self.character_2_sprites[0].pos.y = self.current_round.characters[1].character_position.y;

                if self.current_round.projectiles.len() != self.projectile_sprites.len() {
                    let diff = self.current_round.projectiles.len().abs_diff(self.projectile_sprites.len());
                    if self.current_round.projectiles.len() > self.projectile_sprites.len() {
                        //we need to add the number of new sprites
                        for _ in 0..diff {
                            let (fireball_sprite, mut fireball_render_pass) = setup_fireball(ctx);
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
                    let left = projectile.timer.current_frame * FRAME_WIDTH;

                    let test;
                    match projectile.screen_side {
                        ScreenSide::Left => {
                            test = self.projectile_sprites[index].1.atlas.subsection(left, 0 + FRAME_WIDTH, 0, FRAME_HEIGHT).mirror_y();
                            self.projectile_sprites[index].0[0].pos.x = projectile.position.x * X_SCALE as f32 ;//+ (FRAME_WIDTH as f32 / 2.0) * X_SCALE as f32;
                        },
                        ScreenSide::Right => {
                            test = self.projectile_sprites[index].1.atlas.subsection(left, 0 + FRAME_WIDTH, 0, FRAME_HEIGHT);
                            self.projectile_sprites[index].0[0].pos.x = projectile.position.x * X_SCALE as f32 ;//+ (FRAME_WIDTH as f32 / 2.0) * X_SCALE as f32;
                        }
                    }
                    self.projectile_sprites[index].0[0].texture = test;
                }

                
                if self.effects_sprites.len() != self.current_round.effects.len() {
                    let diff = self.current_round.effects.len().abs_diff(self.effects_sprites.len());

                    if self.current_round.effects.len() > self.effects_sprites.len() {
                        //we need to add the number of new sprites
                        for _ in 0..diff {
                            let (fireball_sprite, mut fireball_render_pass) = setup_light_hit_effect(ctx);
                            fireball_render_pass.atlas = self.light_hit_effect_texture.clone();
                            self.effects_sprites.push((fireball_sprite, fireball_render_pass))
                        }
                    }
                    else {
                        //remove the 
                        self.effects_sprites.truncate(self.effects_sprites.len() - diff);
                    }
                }

                for (index, effect) in self.current_round.effects.iter().enumerate() {
                    let left = effect.current_frame as u32 * EFFECT_FRAME_WIDTH;

                    let test;
                    match effect.screen_side {
                        ScreenSide::Left => {
                            test = self.effects_sprites[index].1.atlas.subsection(left, left + EFFECT_FRAME_WIDTH, 0, 480).mirror_y();
                            self.effects_sprites[index].0[0].pos.x = effect.position_x * X_SCALE as f32 ;//+ (FRAME_WIDTH as f32 / 2.0) * X_SCALE as f32;
                        },
                        ScreenSide::Right => {
                            test = self.effects_sprites[index].1.atlas.subsection(left, left + EFFECT_FRAME_WIDTH, 0, 480);
                            self.effects_sprites[index].0[0].pos.x = effect.position_x * X_SCALE as f32 - (EFFECT_FRAME_WIDTH as f32 / 2.0) * X_SCALE as f32;
                            self.effects_sprites[index].0[0].pos.y = -480.0 * 0.2f32;
                        }
                    }
                    self.effects_sprites[index].0[0].texture = test;
                }

            }

            //MAGIC NUMBER: 145
            let length_of_distance = f32::abs(self.character_2_sprites[0].pos.x - self.character_1_sprites[0].pos.x);
            if length_of_distance > 145.0 {
                self.camera_transform.set().scale = 145.0 / length_of_distance * 5.0;
            }
            else {
                self.camera_transform.set().scale = 5.0;
            }

            self.camera_transform.set().translation.y =  -108.0 + ((5.0 - self.camera_transform.set().scale) * -43.0);

            self.camera_transform.set().translation.x = -(((self.character_2_sprites[0].pos.x + self.character_1_sprites[0].pos.x) + FRAME_WIDTH as f32) / 2.0);

            //Rendering
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
                x:  WIDTH as f32 / 2.0 - 10.0,
                y: HEIGHT as f32 - 90.0,
                max_width: Some(50.0),
                ..Default::default()
            };

            self.ui.timer_text.0.append(
                &self.fonts,
                &layout_settings,
                &[Text {
                    text: &(60 - (self.current_round.round_timer.current_frame / 60)).to_string(),
                    font_index: 0,
                    px: 50.0,
                    color: text_color,
                    depth: 0.0,
                }],
            );

            self.background_sprite_pass.set_transform(self.camera_transform.matrix());
            self.background_sprite_pass.buffer.set_data(&self.background_sprite);
            self.background_sprite_pass.draw(&self.sprite_shader);            

            self.ui.timer_text.0.draw(&self.ui.timer_text.1);

            self.ui.backplate.1.buffer.set_data(&mut self.ui.backplate.0);
            self.ui.backplate.1.draw(&self.sprite_shader);

            for projectile_sprites in self.projectile_sprites.iter_mut() {
                projectile_sprites.1.set_transform(self.camera_transform.matrix());
                projectile_sprites.1.buffer.set_data(&projectile_sprites.0);
                projectile_sprites.1.draw(&self.sprite_shader);
            }


            for effect_sprite in self.effects_sprites.iter_mut() {
                effect_sprite.1.set_transform(self.camera_transform.matrix());
                effect_sprite.1.buffer.set_data(&effect_sprite.0);
                effect_sprite.1.draw(&self.sprite_shader);
            }


            self.sprite_pass_2.set_transform(self.camera_transform.matrix());
            self.sprite_pass_1.set_transform(self.camera_transform.matrix());

            self.sprite_pass_1.buffer.set_data(&self.character_1_sprites);
            self.sprite_pass_1.draw(&self.pallete_sprite_shader);
            
            self.sprite_pass_2.buffer.set_data(&self.character_2_sprites);
            self.sprite_pass_2.draw(&self.pallete_sprite_shader);
            

            
            //Render Health Bars
            let health_ratio_player_one = self.current_round.characters[0].health as f32 / 250.0;
            let health_ratio_player_two = self.current_round.characters[1].health as f32 / 250.0;
            if health_ratio_player_one > 0.95 {
                self.ui.healthbars.0[0].color = RGBA8::GREEN;
            }
            if health_ratio_player_one < 0.95 && health_ratio_player_one > 0.25  {
                self.ui.healthbars.0[0].color = RGBA8::YELLOW;
            }
            else if health_ratio_player_one < 0.25 {
                self.ui.healthbars.0[0].color = RGBA8::RED;
            }

            if health_ratio_player_two > 0.95 {
                self.ui.healthbars.0[1].color = RGBA8::GREEN;                
            }
            else if health_ratio_player_two < 0.95 && health_ratio_player_two > 0.25 {
                self.ui.healthbars.0[1].color = RGBA8::YELLOW;
            }
            else if health_ratio_player_two < 0.25 {
                self.ui.healthbars.0[1].color = RGBA8::RED;
            }

            self.ui.healthbars.0[0].size.x = (480.0 * health_ratio_player_one) as u16;
            self.ui.healthbars.0[1].size.x = (480.0 * health_ratio_player_two) as u16;

            let removed_amount_player_2 = 1.0 - health_ratio_player_two;
            self.ui.healthbars.0[1].pos.x = 160.0 + (480.0 * removed_amount_player_2);

            self.ui.healthbars.1.buffer.set_data(&self.ui.healthbars.0);
            self.ui.healthbars.1.draw(&self.sprite_shader);

        }
    }

    // for each request, call the appropriate function
    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest<GGRSConfig>>) {
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
    pub fn local_input(&self, handle: PlayerHandle) -> NetInput {
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
            if self.local_input.jump_down {
                input |= INPUT_JUMP;
            }
        }
        return NetInput{ input};//input.to_le_bytes();.to_vec();
    }
}