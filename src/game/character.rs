use hashbrown::HashMap;
use ggrs::GameInput;
use super::{INPUT_LEFT, INPUT_RIGHT, INPUT_LIGHT_ATTACK, AnimationState, AnimationConfig};

use storm::*;
use storm::math::AABB2D;
use serde::{Deserialize, Serialize};
use storm::cgmath::Vector2;

pub static CHARACTER_X_SPEED : f32 = 3.0;

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Input {
    pub left_key_down: bool,
    pub right_key_down: bool,
    pub light_attack: bool
}

impl Input {
    pub fn new() -> Input {
        Input {
            left_key_down: false,
            right_key_down: false,
            light_attack: false
        }
    }

    pub fn from_game_input(game_input: GameInput) -> Input {
        Input {
            left_key_down: (game_input.buffer[0] & INPUT_LEFT) != 0,
            right_key_down: (game_input.buffer[0] & INPUT_RIGHT) != 0,
            light_attack: (game_input.buffer[0] & INPUT_LIGHT_ATTACK) != 0
        }
    }

    pub fn key_down(&mut self, keyboard_button: KeyboardButton) {
        match keyboard_button {
            KeyboardButton::Left => {
                self.left_key_down = true;
            },
            KeyboardButton::Right => {
                self.right_key_down = true;
            },
            KeyboardButton::Q => {
                self.light_attack = true;
            },
            _ => {}
        }
    }

    pub fn key_up(&mut self, keyboard_button: KeyboardButton) {
        match keyboard_button {
            KeyboardButton::Left => {
                self.left_key_down = false;
            },
            KeyboardButton::Right => {
                self.right_key_down = false;
            },
            KeyboardButton::Q => {
                self.light_attack = false;
            },
            _ => {}
        }
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum ScreenSide {
    Left,
    Right
}

impl ScreenSide {
    pub fn direction(&self) -> f32 {
        match self {
            &ScreenSide::Left => {
                -1.0
            }
            &ScreenSide::Right => {
                1.0
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone)]
pub enum CharacterState {
    Idle,
    ForwardRun,
    BackwardRun,
    LightAttack
}


#[derive(Serialize, Deserialize)]
pub struct Character {
    pub animation_state: AnimationState,
    pub character_state: CharacterState,
    pub animation_configs: HashMap<AnimationState, AnimationConfig>,
    pub character_position: Vector2<f32>,
    pub character_velocity: Vector2<f32>,
    pub screen_side: ScreenSide
}

impl Character {
    pub fn new(screen_side: ScreenSide) ->Character{
        Character {
            animation_state: AnimationState::Idle,
            character_state: CharacterState::Idle,
            animation_configs: HashMap::new(),
            character_position: Vector2::new(0.0, 0.0),
            character_velocity: Vector2::new(0.0, 0.0),
            screen_side
        }
    }

    pub fn load_animation_config(&mut self, animation_state: AnimationState, animation_config: AnimationConfig) {
        self.animation_configs.insert(animation_state, animation_config);
    }

    pub fn set_character_state(&mut self, new_state: CharacterState) {
        
        self.character_state = new_state;
        //Find out which of the animation states corresponds to the character state 
        //transitioning into
        let animation_state;
        match self.character_state {
            CharacterState::Idle => {
                animation_state = AnimationState::Idle;
            },
            CharacterState::ForwardRun => {
                animation_state = AnimationState::ForwardRun;
            },
            CharacterState::BackwardRun => {
                animation_state = AnimationState::BackwardRun;
            },
            CharacterState::LightAttack => {
                animation_state = AnimationState::LightAttack;
            }
        }

        self.set_animation_state(animation_state);
    }

    pub fn finished_animation_whats_next(&mut self) -> CharacterState {
        //This is a State machine that controls which states a character
        //can move between
        match self.character_state {
            CharacterState::Idle => {
                CharacterState::Idle
            },
            CharacterState::ForwardRun => {
                CharacterState::ForwardRun
            },
            CharacterState::BackwardRun => {
                CharacterState::BackwardRun
            },
            CharacterState::LightAttack => {
                CharacterState::Idle
            }
        }
    }

    pub fn set_animation_state(&mut self, new_state: AnimationState) {
        //Reset the old animation struct before we move onto the new one
        self.animation_state = new_state;
    }

    pub fn get_current_animation_config(&mut self) -> AnimationConfig {
        return *self.animation_configs.get(&self.animation_state).unwrap();
    }
    
    pub fn tick(&mut self, frame_input: Input) {
        //We want an hierarcy of input to handle people button mashing
        //A character should generally be Attacking Over Moving Over Doing Nothing
        if frame_input.light_attack {
            if self.character_state == CharacterState::Idle 
                || self.character_state == CharacterState::ForwardRun 
                || self.character_state == CharacterState::BackwardRun {
                self.set_character_state(CharacterState::LightAttack);
            }
        }

        if self.character_state == CharacterState::Idle || self.character_state == CharacterState::BackwardRun || self.character_state == CharacterState::ForwardRun {
            if frame_input.left_key_down {
                if self.screen_side == ScreenSide::Right {
                    self.set_character_state(CharacterState::ForwardRun);
                }
                else {
                    self.set_character_state(CharacterState::BackwardRun);
                }
            }
            else if frame_input.right_key_down {
                if self.screen_side == ScreenSide::Right {
                    self.set_character_state(CharacterState::BackwardRun);
                }
                else {
                    self.set_character_state(CharacterState::ForwardRun);
                }
            }
        }

        if self.character_state == CharacterState::ForwardRun || self.character_state == CharacterState::BackwardRun {
            if frame_input.light_attack == false && frame_input.left_key_down == false && frame_input.right_key_down == false {
                self.set_character_state(CharacterState::Idle);
            }
        }

        //Then tick the animations to see if we have finished any and we need to be in a new state
        let mut current_animation = self.animation_configs.get_mut(&self.animation_state).unwrap();
        current_animation.sprite_timer.tick();
        if current_animation.sprite_timer.finished() {
            current_animation.sprite_timer.reset();
            current_animation.current_frame += 1;
            //If we have finished the animation move the character into the
            //next state, be that loop(like idle or run)
            //or a steady state like Attack -> Idle
            if current_animation.is_done() {
                current_animation.reset();
                let a = {
                    self.finished_animation_whats_next()
                };
                self.set_character_state(a);
            }
        }
        
        //Lasting state doing state based actions like, moving
        if self.character_state == CharacterState::ForwardRun {
            self.character_velocity.x = -(CHARACTER_X_SPEED * self.screen_side.direction());
        }
        else if self.character_state == CharacterState::BackwardRun {
            self.character_velocity.x = CHARACTER_X_SPEED * self.screen_side.direction();
        }
    }

    pub fn get_current_aabb(&self) -> AABB2D {
        let min_x =  self.character_position.x + 150.0;
        let mix_y = self.character_position.y + 53.0;
        AABB2D { 
            min: Vector2 { x: min_x, y:  mix_y},
            max: Vector2 { x: min_x + 38.0, y:  mix_y + 106.0} 
        }
    }
    
    //A function used to get the information need to lookup a collision box
    pub fn get_collision_box_lookup_info(&self) -> (AnimationState, u32) {
        let mut current_animation = self.animation_configs.get(&self.animation_state).unwrap();
        let current_frame = current_animation.current_frame;
        return (self.animation_state, current_frame);
    }
}