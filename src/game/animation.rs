use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use storm::graphics::Texture;
pub static IDLE_TEXTURE: &[u8] = include_bytes!("../resources/idle.png");
pub static FORWARD_RUN_TEXTURE: &[u8] = include_bytes!("../resources/forward_run.png");
pub static BACKGROUND_RUN_TEXTURE: &[u8] = include_bytes!("../resources/backward_run.png");
pub static LIGHT_ATTACK_TEXTURE: &[u8] = include_bytes!("../resources/light_attack.png");
pub static MEDIUM_ATTACK_TEXTURE: &[u8] = include_bytes!("../resources/medium_attack.png");
pub static HEAVY_ATTACK : &[u8] = include_bytes!("../resources/heavy_attack.png");
pub static LIGHT_HIT_RECOVERY: &[u8] = include_bytes!("../resources/light_hit.png");
pub static BACKGROUND_CASTLE: &[u8] = include_bytes!("../resources/background_castle.png");
pub static BLOCKING: &[u8] = include_bytes!("../resources/blocking.png");
pub static CROUCHED: &[u8] = include_bytes!("../resources/crouched.png");
pub static CROUCHING: &[u8] = include_bytes!("../resources/crouching.png");
pub static LIGHT_CROUCH_ATTACK: &[u8] = include_bytes!("../resources/light_crouch_attack.png");
pub static HEAVY_CROUCH_ATTACK: &[u8] = include_bytes!("../resources/heavy_crouching_attack.png");

use storm::graphics::TextureSection;
pub static FRAME_HEIGHT: u32 =  178;
pub static FRAME_WIDTH: u32 =  290;

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
    HeavyCrouchingAttack
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

#[derive(Serialize, Deserialize, Copy, Clone)]
//A data structure to handle the concept of an animation
pub struct AnimationConfig {
    pub total_length: u32, //The total number of frames
    pub sprite_timer: SpriteTimer, //The number of monotonic frames a single frame will take
    pub current_frame: u32, //The current frame we are on
}

impl AnimationConfig {
    pub fn new(total_length: u32, frame_length: u32) -> AnimationConfig {
        AnimationConfig {
            sprite_timer: SpriteTimer::new(frame_length),
            current_frame: 0,
            total_length
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.sprite_timer.reset();
    }

    pub fn is_done(&self) -> bool {
        return self.current_frame == self.total_length;
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
    pub fn load_animation(&mut self, atlas: &[u8], animation_state: AnimationState) {
        if self.animations.contains_key(&animation_state) {
            panic!("{:?} was already in animation dictionary", animation_state);
        }

        // Use storm to load the image
        let loaded_texture = Texture::from_png(atlas);
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

impl Default for AnimationTextureLibrary {
    fn default() -> AnimationTextureLibrary {
        let mut animation_library = AnimationTextureLibrary::new();
        animation_library.load_animation(IDLE_TEXTURE, AnimationState::Idle);
        animation_library.load_animation(FORWARD_RUN_TEXTURE, AnimationState::ForwardRun);
        animation_library.load_animation(BACKGROUND_RUN_TEXTURE, AnimationState::BackwardRun);
        animation_library.load_animation(LIGHT_ATTACK_TEXTURE, AnimationState::LightAttack);
        animation_library.load_animation(MEDIUM_ATTACK_TEXTURE, AnimationState::MediumAttack);
        animation_library.load_animation(HEAVY_ATTACK, AnimationState::HeavyAttack);
        animation_library.load_animation(LIGHT_HIT_RECOVERY, AnimationState::LightHitRecovery);
        animation_library.load_animation(BLOCKING, AnimationState::Blocking);
        animation_library.load_animation(CROUCHING, AnimationState::Crouching);
        animation_library.load_animation(CROUCHED, AnimationState::Crouched);
        animation_library.load_animation(LIGHT_CROUCH_ATTACK, AnimationState::LightCrouchAttack);
        animation_library.load_animation(HEAVY_CROUCH_ATTACK, AnimationState::HeavyCrouchingAttack);
        return animation_library;
    }
}