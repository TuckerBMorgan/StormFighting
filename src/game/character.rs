use hashbrown::HashMap;
use super::*;
use storm::math::*;
use serde::{Deserialize, Serialize};
use storm::cgmath::Vector2;

pub static CHARACTER_X_SPEED : f32 = 3.0;
pub const FRAME_HISTORY_LENGTH: usize = 15;

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
    MediumAttack,
    HeavyAttack,
    LightHitRecovery,
    Blocking,
    Crouching,
    LightKick,
    MediumKick,
    HeavyKick,
    ForwardDash,
    BackwardDash
}


#[derive(Serialize, Deserialize)]
pub struct Character {
    pub animation_state: AnimationState, //The characters current animation it is playing
    pub character_state: CharacterState, //The current character states
    pub animation_configs: HashMap<AnimationState, AnimationConfig>,//TODO: lift this up one level, it is getting rolled back when it does not need to
    pub character_position: Vector2<f32>, //Where in the world it is
    pub character_velocity: Vector2<f32>, //How far it wants to move this frame
    pub screen_side: ScreenSide, //Which side of the screen it is on
    pub health: u32, //How much health it has
    pub is_crouched: bool, //Is character crouched at the moment, used so we don't have a set of "crouched" states
    pub past_inputs: Vec<ScreenSideAdjustedInput>, //A ring buffer that contains the last 60 input states(about 1 second of input)
}

impl Character {
    pub fn new(screen_side: ScreenSide) -> Character {

        Character {
            animation_state: AnimationState::Idle,
            character_state: CharacterState::Idle,
            animation_configs: HashMap::new(),
            character_position: Vector2::new(0.0, 0.0),
            character_velocity: Vector2::new(0.0, 0.0),
            screen_side,
            health: 250,
            is_crouched: false,
            past_inputs: vec![]
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
            },
            CharacterState::LightKick => {
                animation_state = AnimationState::LightKick;
            },
            CharacterState::MediumKick => {
                animation_state = AnimationState::MediumKick;
            },
            CharacterState::HeavyKick => {
                animation_state = AnimationState::HeavyKick;
            },
            CharacterState::ForwardDash => {
                animation_state = AnimationState::ForwardDash;
            },
            CharacterState::BackwardDash => {
                animation_state = AnimationState::BackwardDash;
            }
        }

        self.set_animation_state(animation_state);
    }

    pub fn finished_animation_whats_next(&mut self) -> CharacterState {
        //This is a State machine that controls which states a character
        //can move between
        match self.character_state {
            CharacterState::ForwardRun => {
                CharacterState::ForwardRun
            },
            CharacterState::BackwardRun => {
                CharacterState::BackwardRun
            },
            _ => {
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
    
    pub fn process_new_input(&mut self, frame_input: ScreenSideAdjustedInput) -> bool {
        if self.past_inputs.len() >= FRAME_HISTORY_LENGTH {
            self.past_inputs.remove(0);
        }
        
        self.past_inputs.push(frame_input);

        let mut dash_start = false;
        let mut dash_confirm = false;
        let mut dash_go = false;
        for element in &self.past_inputs {
            if dash_start == false {
                if element.forward_down == false {
                    dash_start = true;
                }
            }
            else if dash_start && dash_confirm == false {
                if element.forward_down {
                    dash_confirm = true;
                }
            }
            else if dash_start && dash_confirm && dash_go == false {
                if element.forward_down == false {
                    dash_go = true;
                }
            }
            else if dash_go == true {
                if element.forward_down {
                    self.past_inputs.clear();
                    return true;
                }
            }
        }
        return false;
    }
    
    pub fn tick(&mut self, frame_input: Input) {
        let frame_input = ScreenSideAdjustedInput::new(&frame_input, self.screen_side);

        let should_dash = self.process_new_input(frame_input.clone());
        if should_dash {
            self.set_character_state(CharacterState::ForwardDash);
        }
        //We want an hierarcy of input to handle people button mashing
        //A character should generally be Attacking Over Moving Over Doing Nothing
        if self.can_attack() {
            if frame_input.light_attack {
                self.set_character_state(CharacterState::LightAttack);
            }
            else if frame_input.medium_attack {
                self.set_character_state(CharacterState::MediumAttack);
            }
            else if frame_input.heavy_attack {
                self.set_character_state(CharacterState::HeavyAttack);
            }
            else if frame_input.light_kick {
                self.set_character_state(CharacterState::LightKick);
            }
            else if frame_input.medium_kick {
                self.set_character_state(CharacterState::MediumKick);
            }
            else if frame_input.heavy_kick {
                self.set_character_state(CharacterState::HeavyKick);
            }
        }
        //If we are in the normal crouched animation, Idle + IsCrouched, and we are no longer holding the down key
        //Stand the character up
        //TODO: add in a "standing_up" animation state and animation
        if self.character_state == CharacterState::Idle && self.is_crouched && frame_input.down_key_down == false {
            self.is_crouched = false;
            self.set_character_state(CharacterState::Idle);
        }

        if self.character_state == CharacterState::Idle || self.character_state == CharacterState::BackwardRun || self.character_state == CharacterState::ForwardRun {
            if frame_input.forward_down {
                self.set_character_state(CharacterState::ForwardRun);
            }
            else if frame_input.backward_down {
                self.set_character_state(CharacterState::BackwardRun);
            }
            else if frame_input.down_key_down {
                if self.is_crouched == false {
                    self.is_crouched = true;
                    self.set_character_state(CharacterState::Crouching);
                }
            }
        }

        if self.character_state == CharacterState::ForwardRun || self.character_state == CharacterState::BackwardRun {
            if frame_input.light_attack == false && frame_input.forward_down == false && frame_input.backward_down == false {
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
        else if self.character_state == CharacterState::ForwardDash {
            self.character_velocity.x = -(CHARACTER_X_SPEED * self.screen_side.direction()) * 2.0;
        }
        else if self.character_state == CharacterState::BackwardRun {
            self.character_velocity.x = CHARACTER_X_SPEED * self.screen_side.direction();
        }
        else if self.character_state == CharacterState::LightHitRecovery {
            self.character_velocity.x = CHARACTER_X_SPEED * self.screen_side.direction();
        }
        else {
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
        //TODO: REMOVE THE MAGIC NUMBERS
        return AABB2D::new(self.character_position.x + 131.0, self.character_position.y + 57.0, 
                           self.character_position.x + 131.0 + 33.0, self.character_position.y + 57.0 + 103.0);
    }

    pub fn get_current_damage(&self) -> u32 {
        match self.character_state  {
            CharacterState::LightAttack 
                | CharacterState::LightKick => {
                    return 10;
            },
            CharacterState::MediumAttack 
                | CharacterState::MediumKick => {
                    return 20;
            },
            CharacterState::HeavyAttack 
                | CharacterState::HeavyKick => {
                return 30;
            }
            _ => {
                return 0;
            }
        }
    }

    #[inline(always)]
    pub fn can_attack(&self) -> bool {
        return self.character_state == CharacterState::Idle 
                || self.character_state == CharacterState::ForwardRun 
                || self.character_state == CharacterState::BackwardRun
                || self.character_state == CharacterState::ForwardDash;
    }
}

//TODO: make this data driven, this is tedious and error prone
impl Default for Character {
    fn default() -> Self {
        let mut character = Character::new(ScreenSide::Left);
        character.load_animation_config(AnimationState::Idle,                 AnimationConfig::new(10, 4));
        character.load_animation_config(AnimationState::ForwardRun,           AnimationConfig::new(12, 4));
        character.load_animation_config(AnimationState::BackwardRun,          AnimationConfig::new(10, 4));
        character.load_animation_config(AnimationState::LightAttack,          AnimationConfig::new(5, 4));
        character.load_animation_config(AnimationState::MediumAttack,         AnimationConfig::new(8, 4));
        character.load_animation_config(AnimationState::HeavyAttack,          AnimationConfig::new(11, 4));
        character.load_animation_config(AnimationState::LightHitRecovery,     AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Blocking,             AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Crouched,             AnimationConfig::new(4, 4));
        character.load_animation_config(AnimationState::Crouching,            AnimationConfig::new(2, 4));
        character.load_animation_config(AnimationState::LightCrouchAttack,    AnimationConfig::new(5, 4));
        character.load_animation_config(AnimationState::HeavyCrouchingAttack, AnimationConfig::new(9, 4));
        character.load_animation_config(AnimationState::LightKick,            AnimationConfig::new(6, 4));
        character.load_animation_config(AnimationState::MediumKick,           AnimationConfig::new(8, 4));
        character.load_animation_config(AnimationState::HeavyKick,            AnimationConfig::new(13, 4));
        character.load_animation_config(AnimationState::ForwardDash,          AnimationConfig::new(6, 4));
        character.load_animation_config(AnimationState::BackwardDash,         AnimationConfig::new(6, 4));
        return character;
    }
}