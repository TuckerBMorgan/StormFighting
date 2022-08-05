use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use storm::graphics::Texture;
use storm::graphics::TextureFiltering;
use storm::graphics::TextureSection;
use storm::*;
use crate::FighthingApp;

// TODO: load these normally
pub static BACKGROUND_CASTLE: &[u8] = include_bytes!("../../resources/background_castle.png");
pub static UI_BACKPLATE: &[u8] = include_bytes!("../../resources/health_and_time_ui.png");
pub static GREYSCALE_HEALTH_BAR_GRADIANT: &[u8] = include_bytes!("../../resources/greyscale_health_bar.png");
pub static BUTTON: &[u8] = include_bytes!("../../resources/button.png");

pub const FRAME_HEIGHT: u32 =  178;
pub const FRAME_WIDTH: u32 =  290;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AnimationState {
    Idle,
    ForwardRun,
    BackwardRun,
    LightAttack,
    MediumAttack,
    HeavyAttack,
    LightHitRecovery,
    Crouched,
    Crouching,
    Blocking,
    LightCrouchAttack,
    HeavyCrouchingAttack,
    LightKick,
    MediumKick,
    HeavyKick,
    ForwardDash,
    BackwardDash,
    Special1,
    Won,
    Lost,
    Jump,
    Parry,
    ForwardJump
}

// TODO: Just all of this :( I feel bad for this
// I would like for there to be a way to have animations represented as a single value(enum), while at the same time being
// able to use their names as a look up for animations. 
impl AnimationState {
    pub fn from_string(value: &String) -> AnimationState {
        if value == "Idle" {
            return AnimationState::Idle;
        }
        if value == "ForwardRun" {
            return AnimationState::ForwardRun;
        }
        if value == "BackwardRun" {
            return AnimationState::BackwardRun;
        }
        if value == "LightAttack" {
            return AnimationState::LightAttack;
        }
        if value == "MediumAttack" {
            return AnimationState::MediumAttack;
        }
        if value == "HeavyAttack" {
            return AnimationState::HeavyAttack;
        }
        if value == "LightHitRecovery" {
            return AnimationState::LightHitRecovery;
        }
        if value == "Crouched" {
            return AnimationState::Crouched;
        }
        if value == "Crouching" {
            return AnimationState::Crouching;
        }
        if value == "Blocking" {
            return AnimationState::Blocking;
        }
        if value == "LightCrouchAttack" {
            return AnimationState::LightCrouchAttack;
        }
        if value == "HeavyCrouchingAttack" {
            return AnimationState::HeavyCrouchingAttack;
        }
        if value == "LightKick" {
            return AnimationState::LightKick;
        }
        if value == "MediumKick" {
            return AnimationState::MediumKick;
        }
        if value == "HeavyKick"{
            return AnimationState::HeavyKick;
        }
        if value == "ForwardDash" {
            return AnimationState::ForwardDash;
        }
        if value == "BackwardDash" {
            return AnimationState::BackwardDash;
        }
        if value == "Special1" {
            return AnimationState::Special1;
        }
        if value == "Won" {
            return AnimationState::Won;
        }
        if value == "Lost" {
            return AnimationState::Lost;
        }
        if value == "Jump" {
            return AnimationState::Jump;
        }
        if value == "Parry" {
            return AnimationState::Parry;
        }
        if value == "ForwardJump" {
            return AnimationState::ForwardJump;
        }
        panic!("{:?} is an unknow animation state", value);
    }

    pub fn to_string(&self) -> String {
        match *self {
            AnimationState::Idle => {
                return String::from("Idle");
            }
            AnimationState::ForwardRun => {
                return String::from("ForwardRun");
            }
            AnimationState::BackwardRun => {
                return String::from("BackwardRun");
            }
            AnimationState::LightAttack => {
                return String::from("LightAttack");
            }
            AnimationState::MediumAttack => {
                return String::from("MediumAttack");
            }
            AnimationState::HeavyAttack => {
                return String::from("HeavyAttack");
            }
            AnimationState::LightHitRecovery => {
                return String::from("LightHitRecovery");
            }
            AnimationState::Crouched => {
                return String::from("Crouched");
            }
            AnimationState::Crouching => {
                return String::from("Crouching");
            }
            AnimationState::Blocking => {
                return String::from("Blocking");
            }
            AnimationState::LightCrouchAttack => {
                return String::from("LightCrouchAttack");
            }
            AnimationState::HeavyCrouchingAttack => {
                return String::from("HeavyCrouchingAttack");
            }
            AnimationState::LightKick => {
                return String::from("LightKick");
            }
            AnimationState::MediumKick => {
                return String::from("MediumKick");
            }
            AnimationState::HeavyKick => {
                return String::from("HeavyKick");
            }
            AnimationState::ForwardDash => {
                return String::from("ForwardDash");
            }
            AnimationState::BackwardDash => {
                return String::from("BackwardDash");
            }
            AnimationState::Special1 => {
                return String::from("Special1");
            }
            AnimationState::Won => {
                return String::from("Won");
            }
            AnimationState::Lost => {
                return String::from("Lost");
            }
            AnimationState::Jump => {
                return String::from("Jump");
            }
            AnimationState::Parry => {
                return String::from("Parry");
            }
            AnimationState::ForwardJump => {
                return String::from("ForwardJump");
            }
        }
    }
}


#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Copy, Clone)]
//A frame number based timer for sprites IE: Does not use delta timer/real time it is an monotonic timer
pub struct SpriteTimer {
    pub total_frames: u32,
    pub current_frame: u32,
    pub finished: bool,
}

impl SpriteTimer {
    pub fn new(total_frames: u32) -> SpriteTimer {
        SpriteTimer {
            total_frames,
            current_frame: 0,
            finished: false,
        }
    }

    //How you advance the timer 
    pub fn tick(&mut self) {
        self.finished = false;
        self.current_frame += 1;
        if self.current_frame == self.total_frames {
            self.finished = true;
            self.current_frame = 0;
        }
    }

    //Call to see if this timer has finished it timer
    pub fn finished(&mut self) -> bool {
        self.finished
    }

    //Resets the state of the timer, but not total_frames
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.finished = false;
    }
}

#[derive(Serialize, Deserialize, Clone)]
//A data structure to handle the concept of an animation
pub struct AnimationConfig {
    pub sprite_timer: SpriteTimer, //The number of monotonic frames a single frame will take
    pub current_frame: u32, //The current frame we are on
    pub frame_times: Vec<u8>
}

impl AnimationConfig {
    pub fn new(frame_times: Vec<u8>) -> AnimationConfig {
        AnimationConfig {
            sprite_timer: SpriteTimer::new(frame_times[0] as u32),
            current_frame: 0,
            frame_times
        }
    }

    pub fn advance_to_next_frame(&mut self) {
        self.sprite_timer = SpriteTimer::new(self.frame_times[self.current_frame as usize] as u32);
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.sprite_timer.reset();
    }

    pub fn is_done(&self) -> bool {
        return self.current_frame == self.frame_times.len() as u32;
    }
}

pub struct AnimationTextureLibrary {
    pub animations: HashMap<AnimationState, Texture>,
}

impl AnimationTextureLibrary {
    pub fn new() -> AnimationTextureLibrary {
        AnimationTextureLibrary {
            animations: HashMap::new()
        }
    }

    // Given atlas(a u8 representation of the image we want) and the animation state we want
    // Build a mapping between the two so that we can look it up later
    pub fn load_animation(&mut self, atlas: &[u8], animation_state: AnimationState, ctx: &mut Context<FighthingApp>) {
        if self.animations.contains_key(&animation_state) {
            panic!("{:?} was already in animation dictionary", animation_state);
        }

        // Use storm to load the image
        let loaded_texture = Texture::from_png(ctx, atlas, TextureFiltering::none());
        self.animations.insert(animation_state, loaded_texture);
    }

    //Returns a immutable reference to the underalying loaded atlas, which has been loaded by Storm
    pub fn get_atlas_for_animation(&self, animation_state: AnimationState) -> Texture {
        let current_animation = self.animations.get(&animation_state).unwrap();
        return current_animation.clone();
    }

    //Returns the subsection of an atlas used for rendering
    //Makes an assumption that all frames for all animations are of the same width
    //returns the section starting at frame_number * FRAME_WIDTH to frame_number * FRAME_WIDTH + FRAME_WIDTH
    pub fn get_atlas_subsection(& self, animation: AnimationState, frame_number: u32) -> TextureSection {
        let left = frame_number * FRAME_WIDTH;
        return self.animations.get(&animation).unwrap().subsection(left, left + FRAME_WIDTH, 0, FRAME_HEIGHT);
    }
}
