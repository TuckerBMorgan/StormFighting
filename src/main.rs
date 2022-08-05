#![feature(int_abs_diff)]
use core::convert::{From};
use core::time::Duration;

extern crate simplelog;
use simplelog::*;
use hashbrown::HashMap;
use storm::graphics::*;
use storm::event::*;
use storm::*;

mod game;
mod shaders;

use game::*;
use shaders::*;

static FONT: &[u8] = include_bytes!("../resources/gomarice_game_continue_02.ttf");
static FIREBALL: &[u8] = include_bytes!("../resources/fireball_main.png");

#[cfg(target_arch = "wasm32")]
static RESOURCE_PATH : &'static str = "../resources/";

#[cfg(not(target_arch = "wasm32"))]
static RESOURCE_PATH : &'static str = "./resources/";

const WIDTH : usize =  1440;
const HEIGHT : usize =  1080;

const X_SCALE : u16 = 1;

#[derive(Eq, PartialEq)]
pub enum AppState {
    Play,
    Pause,
    Debug
}
#[derive(Eq, PartialEq)]
pub enum GameState {
    Loading,
    Menu,
    Game
}

fn main() {
    //I am initing this logger to avoid an error on mac
    let _ = SimpleLogger::init(LevelFilter::Warn, Config::default());
    // Create the engine context and describe the window.
    start::<FighthingApp>(
        WindowSettings {
            title: String::from("Storm Fighting"),
            display_mode: DisplayMode::Windowed {
                width: WIDTH as i32,
                height: HEIGHT as i32,
                resizable: false,
            },
            vsync: Vsync::Disabled,
        }
    );
}

pub struct FighthingApp {
    pub game_state: GameState,
    pub transitioning: bool,
    pub game: Option<Game<'static>>,
    pub menu: Option<Menu>
}

impl App for FighthingApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 60.0)));
        let game_state = GameState::Loading;
        let game: Option<Game> = None;
        let menu : Option<Menu> = None;

        ctx.read(&Menu::files_needed_to_start(), Menu::files_loaded);

        FighthingApp {
            game_state,
            game,
            menu,
            transitioning: false
        }
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, _delta: f32) {
        match self.game_state {
            GameState::Game => {
                if self.game.is_some() {
                    self.game.as_mut().unwrap().update(ctx);
                }
            },
            GameState::Menu => {
                let desired_game_state = self.menu.as_mut().unwrap().tick(ctx);
                if desired_game_state != GameState::Menu && self.transitioning == false {
                    self.game_state = desired_game_state;
                    self.transitioning = true;
                    //self.game = Some(Game::load_basic_game(ctx));

                    ctx.read(&[String::from(RESOURCE_PATH) + &String::from("ryu_character_sheet.json")], move |ctx, _app, assets|{
                        for asset in assets {
                            match asset.result {
                                Ok(a_thing) => {
                                    let character_sheet : CharacterSheet = serde_json::from_str(&String::from_utf8(a_thing).unwrap()).unwrap();

                                    let mut images_to_load: Vec<String> = vec![];
                                    let mut names_of_animations = vec![];
                                    for (name, animation_data) in character_sheet.animations.clone() {
                                        let k = animation_data.image_file_location;
                                        names_of_animations.push(name.clone());
                                        images_to_load.push(String::from(RESOURCE_PATH) + &k);
                                    }
                                    ctx.read(&images_to_load[..], move |ctx, app, assets|{
                                        let mut animation_texture_library = AnimationTextureLibrary::new();
                                        for (index, name) in names_of_animations.iter().enumerate() {
                                            match &assets[index].result {
                                                Ok(atlas) => {
                                                    let animation_state : AnimationState = AnimationState::from_string(name);
                                                    animation_texture_library.load_animation(&atlas, animation_state, ctx)
                                                },
                                                Err(e) => {
                                                    panic!("error loading image {:?} {:?}", e, assets[index].relative_path);
                                                }
                                            }
                                        }
                                        app.transitioning = false;

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
                                        animation_for_character_state_library.insert(CharacterState::Jump, AnimationStateForCharacterState::new(AnimationState::Jump, AnimationState::Jump));
                                        animation_for_character_state_library.insert(CharacterState::Parry, AnimationStateForCharacterState::new(AnimationState::Parry, AnimationState::Parry));
                                        animation_for_character_state_library.insert(CharacterState::Parried, AnimationStateForCharacterState::new(AnimationState::LightHitRecovery, AnimationState::LightHitRecovery));
                                        animation_for_character_state_library.insert(CharacterState::ForwardJump, AnimationStateForCharacterState::new(AnimationState::ForwardJump, AnimationState::ForwardJump));
                                        
                                        let animation_state = vec![
                                            AnimationState::Idle,
                                            AnimationState::ForwardRun,
                                            AnimationState::BackwardRun,
                                            AnimationState::LightAttack,
                                            AnimationState::MediumAttack,
                                            AnimationState::HeavyAttack,
                                            AnimationState::LightHitRecovery,
                                            AnimationState::Crouched,
                                            AnimationState::Crouching,
                                            AnimationState::Blocking,
                                            AnimationState::LightCrouchAttack,
                                            AnimationState::HeavyCrouchingAttack,
                                            AnimationState::LightKick,
                                            AnimationState::MediumKick,
                                            AnimationState::HeavyKick,
                                            AnimationState::ForwardDash,
                                            AnimationState::BackwardDash,
                                            AnimationState::Special1,
                                            AnimationState::Won,
                                            AnimationState::Lost,
                                            AnimationState::Jump,
                                            AnimationState::Parry,
                                            AnimationState::ForwardJump
                                        ];
                                        
                                        let mut animation_configs = HashMap::new();
                                        for state in animation_state {
                                            animation_configs.insert(state, AnimationConfig::new(character_sheet.animations.get(&state.to_string()).unwrap().frame_lengths.clone()));
                                        }

                                        let game_config = GameConfig::new(CollisionLibrary::new_from_sheet(&character_sheet), ComboLibrary::default(), animation_texture_library, animation_for_character_state_library, animation_configs, character_sheet.clone());

                                        app.game = Some(Game::load_game_with_config(ctx, game_config));
                                        app.game_state = GameState::Game;
                                    });
                                },
                                Err(e ) => {
                                    panic!("{:?}", e);
                                }
                            }
                        }
                    });
                }
            },
            _ => {
            }
        }
    }

    
    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            _ => {
                if self.game.is_some() {
                    self.game.as_mut().unwrap().key_down(key);
                }
            }
        }
    }

    fn on_key_released(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            _ => {
                if self.game.is_some() {
                    self.game.as_mut().unwrap().key_up(key);
                }
            }
        }
    }

    fn on_cursor_pressed(
        &mut self,
        _ctx: &mut Context<Self>,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        normalized_pos: cgmath::Vector2<f32>,
    ) {
        match self.game_state {
            GameState::Menu => {
                self.menu.as_mut().unwrap().mouse_down(normalized_pos);
            },
             _ => {
                 
             }
        }
    }

    fn on_cursor_released(
        &mut self,
        _ctx: &mut Context<Self>,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        normalized_pos: cgmath::Vector2<f32>,
    ) {
        match self.game_state {
            GameState::Menu => {
                self.menu.as_mut().unwrap().mouse_up(normalized_pos);
            },
             _ => {
                 
             }
        }
    }
}
