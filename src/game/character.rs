use hashbrown::HashMap;
use ggrs::GameInput;
use super::{INPUT_LEFT, INPUT_RIGHT, INPUT_DOWN, INPUT_LIGHT_ATTACK, INPUT_MEDIUM_ATTACK, INPUT_HEAVY_ATTACK, AnimationState, AnimationConfig};
use storm::*;
use storm::math::*;
use serde::{Deserialize, Serialize};
use storm::cgmath::Vector2;

pub static CHARACTER_X_SPEED : f32 = 3.0;

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Input {
    pub left_key_down: bool,
    pub right_key_down: bool,
    pub down_key_down: bool,
    pub light_attack: bool,
    pub medium_attack: bool,
    pub heavy_attack: bool
}

impl Input {
    pub fn new() -> Input {
        Input {
            left_key_down: false,
            right_key_down: false,
            down_key_down: false,
            light_attack: false,
            medium_attack: false,
            heavy_attack: false
        }
    }

    pub fn from_game_input(game_input: GameInput) -> Input {
        Input {
            left_key_down:  (game_input.buffer[0] & INPUT_LEFT) != 0,
            right_key_down: (game_input.buffer[0] & INPUT_RIGHT) != 0,
            down_key_down:  (game_input.buffer[0] & INPUT_DOWN) != 0,
            light_attack:   (game_input.buffer[0] & INPUT_LIGHT_ATTACK) != 0,
            medium_attack:   (game_input.buffer[0] & INPUT_MEDIUM_ATTACK) != 0,
            heavy_attack: (game_input.buffer[0] & INPUT_HEAVY_ATTACK) != 0
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
            KeyboardButton::Down => {
                self.down_key_down = true;
            },
            KeyboardButton::W => {
                self.medium_attack = true;
            },
            KeyboardButton::E => {
                self.heavy_attack = true;
            }
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
            KeyboardButton::Down => {
                self.down_key_down = false;
            }
            KeyboardButton::W => {
                self.medium_attack = false;
            },
            KeyboardButton::E => {
                self.heavy_attack = false;
            }
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
    LightAttack,
    LightHitRecovery,
    Blocking,
    Crouching,
    MediumAttack,
    HeavyAttack
}


#[derive(Serialize, Deserialize)]
pub struct Character {
    pub animation_state: AnimationState,
    pub character_state: CharacterState,
    pub animation_configs: HashMap<AnimationState, AnimationConfig>,
    pub character_position: Vector2<f32>,
    pub character_velocity: Vector2<f32>,
    pub screen_side: ScreenSide,
    pub health: u32,
    pub is_crouched: bool
}

impl Character {
    pub fn new(screen_side: ScreenSide) ->Character{
        Character {
            animation_state: AnimationState::Idle,
            character_state: CharacterState::Idle,
            animation_configs: HashMap::new(),
            character_position: Vector2::new(0.0, 0.0),
            character_velocity: Vector2::new(0.0, 0.0),
            screen_side,
            health: 100,
            is_crouched: false
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
                if self.is_crouched {
                    animation_state = AnimationState::Crouched;
                }
                else {
                    animation_state = AnimationState::Idle;
                }
            },
            CharacterState::ForwardRun => {
                animation_state = AnimationState::ForwardRun;
            },
            CharacterState::BackwardRun => {
                animation_state = AnimationState::BackwardRun;
            },
            CharacterState::LightAttack => {
                if self.is_crouched {
                    animation_state = AnimationState::LightCrouchAttack;
                }
                else {
                    animation_state = AnimationState::LightAttack;
                }
            },
            CharacterState::LightHitRecovery => {
                animation_state = AnimationState::LightHitRecovery;
            },
            CharacterState::Blocking => {
                animation_state = AnimationState::Blocking;
            },
            CharacterState::Crouching => {
                animation_state = AnimationState::Crouching;
            },
            CharacterState::MediumAttack => {
                if self.is_crouched {
                    animation_state = AnimationState::LightCrouchAttack;
                }
                else {
                    animation_state = AnimationState::MediumAttack;
                }
            },
            CharacterState::HeavyAttack => {
                if self.is_crouched {
                    animation_state = AnimationState::HeavyCrouchingAttack;
                }
                else {
                    animation_state = AnimationState::HeavyAttack;
                }

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
            },
            CharacterState::MediumAttack => {
                CharacterState::Idle
            },
            CharacterState::HeavyAttack => {
                CharacterState::Idle
            },
            CharacterState::LightHitRecovery => {
                CharacterState::Idle
            },
            CharacterState::Blocking => {
                CharacterState::Idle
            },
            CharacterState::Crouching => {
                CharacterState::Idle
            }
        }
    }

    pub fn set_animation_state(&mut self, new_state: AnimationState) {
        //Reset the old animation struct before we move onto the new one
        self.animation_state = new_state;
    }

    pub fn get_current_animation_config(&self) -> AnimationConfig {
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
        else if frame_input.medium_attack {
            if self.character_state == CharacterState::Idle 
                || self.character_state == CharacterState::ForwardRun 
                || self.character_state == CharacterState::BackwardRun {
                self.set_character_state(CharacterState::MediumAttack);
            }
        }
        else if frame_input.heavy_attack {
            if self.character_state == CharacterState::Idle 
                || self.character_state == CharacterState::ForwardRun 
                || self.character_state == CharacterState::BackwardRun {
                self.set_character_state(CharacterState::HeavyAttack);
            }
        }
        //If we are in the normal crouched animation, Idle + IsCrouched, and we are no longer holding the down key
        //Stand the character up
        //TODO: add in a "standing_up" state and animation
        if self.character_state == CharacterState::Idle && self.is_crouched && frame_input.down_key_down == false {
            self.is_crouched = false;
            self.set_character_state(CharacterState::Idle);
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
            else if frame_input.down_key_down {
                if self.is_crouched == false {
                    self.is_crouched = true;
                    self.set_character_state(CharacterState::Crouching);
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
                let new_state = {
                    self.finished_animation_whats_next()
                };
                self.set_character_state(new_state);
            }
        }
        
        //Lasting state doing state based actions like, moving
        if self.character_state == CharacterState::ForwardRun {
            self.character_velocity.x = -(CHARACTER_X_SPEED * self.screen_side.direction());
        }
        else if self.character_state == CharacterState::BackwardRun {
            self.character_velocity.x = CHARACTER_X_SPEED * self.screen_side.direction();
        }
        else if self.character_state == CharacterState::LightHitRecovery {
            self.character_velocity.x = CHARACTER_X_SPEED * self.screen_side.direction();
        }
        else if self.character_state == CharacterState::Idle {
            self.character_velocity.x = 0.0;
        }
        else if self.character_state == CharacterState::LightAttack  
                || self.character_state == CharacterState::MediumAttack 
                || self.character_state == CharacterState::HeavyAttack 
        {
            self.character_velocity.x = 0.0;
        }
        else if self.is_crouched {
            self.character_velocity.x = 0.0;
        }
    }

    //Returns if the character is in the subset of states that are "damageable" ei: Non recovery states
    //At some point we should remove this, and simply  have frames marked as "invulnerable"
    //TODO: do above comment
    pub fn is_in_damageable_state(&self) -> bool {
        return self.character_state != CharacterState::LightHitRecovery;
    }
    
    //Do an amount of damage to the character
    pub fn do_damage(&mut self, amount: u32) {
        match self.character_state {
            CharacterState::Blocking => {
                self.health -= amount / 10;
                self.set_character_state(CharacterState::Blocking);
            }
            _ => {
                self.health -= amount;
                self.set_character_state(CharacterState::LightHitRecovery);
            }
        }
    }

    //A function used to get the information need to lookup a collision box
    pub fn get_collision_box_lookup_info(&self) -> (AnimationState, u32) {
        let current_animation = self.animation_configs.get(&self.animation_state).unwrap();
        let current_frame = current_animation.current_frame;
        return (self.animation_state, current_frame);
    }



    // The "Walk box" is what AABB used to move the character
    // It is not part of the collision system used for combat
    pub fn get_walk_box(&self)  -> AABB2D {
        /*
        //If the character is crouched the walk box is shorter
        if self.is_crouched {
            return AABB2D::new(self.character_position.x + 131.0, self.character_position.y + 30.0, 
                self.character_position.x + 131.0 + 33.0, self.character_position.y + 30.0 + 103.0);
        }
        */
        //These numbers are ones that I grabbed off of my old first passs on hit boxes
        //They need to be data driven at some point
        //TODO: REMOVE THE MAGJIC NUMBERS
        return AABB2D::new(self.character_position.x + 131.0, self.character_position.y + 57.0, 
                           self.character_position.x + 131.0 + 33.0, self.character_position.y + 57.0 + 103.0);
    }
}

impl Default for Character {
    fn default() -> Self {
        let mut character = Character::new(ScreenSide::Left);
        character.load_animation_config(AnimationState::Idle, AnimationConfig::new(10, 4));
        character.load_animation_config(AnimationState::ForwardRun, AnimationConfig::new(12, 4));
        character.load_animation_config(AnimationState::BackwardRun, AnimationConfig::new(10, 4));
        character.load_animation_config(AnimationState::LightAttack, AnimationConfig::new(5, 4));
        character.load_animation_config(AnimationState::MediumAttack, AnimationConfig::new(8, 4));
        character.load_animation_config(AnimationState::HeavyAttack, AnimationConfig::new(11, 4));
        character.load_animation_config(AnimationState::LightHitRecovery, AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Blocking, AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Crouched, AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Crouching, AnimationConfig::new(2, 4));
        character.load_animation_config(AnimationState::LightCrouchAttack, AnimationConfig::new(5, 4));
        character.load_animation_config(AnimationState::HeavyCrouchingAttack, AnimationConfig::new(9, 4));
        return character;
    }
}