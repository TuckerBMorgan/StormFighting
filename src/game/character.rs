use super::*;
use storm::math::*;
use serde::{Deserialize, Serialize};
use storm::cgmath::Vector2;

pub const CHARACTER_X_SPEED : f32 = 5.0;
pub const FRAME_HISTORY_LENGTH: usize = 30;

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

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum CharacterState {
    Idle,
    ForwardRun,
    BackwardRun,
    LightAttack,
    MediumAttack,
    HeavyAttack,
    LightHitRecovery,
    MediumHitRecovery,
    Blocking,
    Crouching,
    LightKick,
    MediumKick,
    HeavyKick,
    ForwardDash,
    BackwardDash,
    Special1,
    Won,
    Lost,
    Jump,
    Parry
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum CharacterAction {
    None,
    MoveForward,
    MoveBackward,
    DashForward,
    DashBackward,
    LightAttack,
    MediumAttack,
    HeavyAttack,
    LightKick,
    MediumKick,
    HeavyKick,
    Crouch,
    Special1,
    Jump,
    Parry
}

pub struct AnimationStateForCharacterState {
    pub crouched: AnimationState,
    pub standing: AnimationState
}

impl AnimationStateForCharacterState {
    pub fn new(crouched: AnimationState, standing: AnimationState) -> AnimationStateForCharacterState {
        AnimationStateForCharacterState {
            crouched,
            standing
        }
    }
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Character {
    pub animation_state: AnimationState, //The characters current animation it is playing
    pub character_state: CharacterState, //The current character states
    pub current_animation: AnimationConfig,//TODO: lift this up one level, it is getting rolled back when it does not need to
    pub character_position: Vector2<f32>, //Where in the world it is
    pub character_velocity: Vector2<f32>, //How far it wants to move this frame
    pub screen_side: ScreenSide, //Which side of the screen it is on
    pub health: u32, //How much health it has
    pub is_crouched: bool, //Is character crouched at the moment, used so we don't have a set of "crouched" states
    pub past_inputs: Vec<ScreenSideAdjustedInput>, //A buffer that contains the last FRAME_HISTORY_LENGTH input states
    pub done: bool
}

impl Character {
    pub fn new(screen_side: ScreenSide) -> Character {
        Character {
            animation_state: AnimationState::Idle,
            character_state: CharacterState::Idle,
            current_animation: AnimationConfig::new(vec![1;1]),
            character_position: Vector2::new(0.0, 0.0),
            character_velocity: Vector2::new(0.0, 0.0),
            screen_side,
            health: 250,
            is_crouched: false,
            past_inputs: vec![],
            done: false
        }
    }

    pub fn set_character_state(&mut self, new_state: CharacterState, game_config: &GameConfig) {

        self.character_state = new_state;
        let animation_state;
        if self.is_crouched {
            animation_state = game_config.animation_for_character_state_library.get(&self.character_state).unwrap().crouched;
        }
        else {
            animation_state = game_config.animation_for_character_state_library.get(&self.character_state).unwrap().standing;
        }
        self.current_animation = game_config.animation_configs.get(&animation_state).unwrap().clone();
        self.current_animation.reset();
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
        return self.current_animation.clone();
    }
    
    pub fn process_new_input(&mut self, frame_input: ScreenSideAdjustedInput, combo_library: &mut ComboLibrary) -> CharacterAction {

        if self.past_inputs.len() >= FRAME_HISTORY_LENGTH {
            self.past_inputs.remove(0);
        }

        combo_library.reset();
        self.past_inputs.push(frame_input);

        for element in &self.past_inputs {
            for combo in combo_library.combos.iter_mut() {
                match combo.process_input(element) {
                    Some(character_action) => {
                        self.past_inputs.clear();
                        return character_action;
                    }
                    None => {

                    }
                }
            }
        }

        if frame_input.light_attack {
            return CharacterAction::LightAttack;
        }
        else if frame_input.medium_attack {
            return CharacterAction::MediumAttack;
        }
        else if frame_input.heavy_attack {
            return CharacterAction::HeavyAttack;
        }
        else if frame_input.light_kick {
            return CharacterAction::LightKick;
        }
        else if frame_input.medium_kick {
            return CharacterAction::MediumKick;
        }
        else if frame_input.heavy_kick {
            return CharacterAction::HeavyKick;
        }
        else if frame_input.forward_down {
            return CharacterAction::MoveForward;
        }
        else if frame_input.backward_down {
            return CharacterAction::MoveBackward;
        }
        else if frame_input.down_key_down {
            return CharacterAction::Crouch;
        }
        else if frame_input.jump {
            //TODO: make jump work again
            return CharacterAction::Parry;
        }
        

        return CharacterAction::None;
    }


    //Returns if the character is in the subset of states that are "damageable" ei: Non recovery states
    //At some point we should remove this, and simply  have frames marked as "invulnerable"
    //TODO: do above comment
    pub fn is_in_damageable_state(&self) -> bool {
        return self.character_state != CharacterState::LightHitRecovery && self.character_state != CharacterState::Jump;
    }
    //A function used to get the information need to lookup a collision box
    pub fn get_collision_box_lookup_info(&self) -> (AnimationState, u32) {
        let current_frame = self.current_animation.current_frame;
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
                           self.character_position.x + 131.0 + 33.0, self.character_position.y + 67.0);
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
        let is_in_idle_state = self.character_state == CharacterState::Idle 
            || self.character_state == CharacterState::ForwardRun 
            || self.character_state == CharacterState::BackwardRun
            || self.character_state == CharacterState::ForwardDash;
        return is_in_idle_state;
    }
}

//TODO: make this data driven, this is tedious and error prone
impl Default for Character {
    fn default() -> Self {
        return Character::new(ScreenSide::Left);
    }
}