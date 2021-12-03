use storm::*;
use hashbrown::HashMap;

use serde::{Deserialize, Serialize};

pub static IDLE_TEXTURE: &[u8] = include_bytes!("../resources/idle.png");
pub static FORWARD_RUN_TEXTURE: &[u8] = include_bytes!("../resources/forward_run.png");
pub static BACKGROUND_RUN_TEXTURE: &[u8] = include_bytes!("../resources/backward_run.png");
pub static LIGHT_ATTACK_TEXTURE: &[u8] = include_bytes!("../resources/light_attack.png");

pub static FRAME_HEIGHT: u32 =  178;
pub static FRAME_WIDTH: u32 =  290;

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum AnimationState {
    Idle,
    ForwardRun,
    BackwardRun,
    LightAttack
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
    pub animations: HashMap<AnimationState, Texture<RGBA8>>,
}

impl AnimationTextureLibrary {
    pub fn new() -> AnimationTextureLibrary {
        AnimationTextureLibrary {
            animations: HashMap::new()
        }
    }

    // Given atlas(a u8 representation of the image we want) and the animation state we want
    // Build a mapping between the two so that we can look it up later
    pub fn load_animation(&mut self, ctx: &mut Context, atlas: &[u8], animation_state: AnimationState) {
        if self.animations.contains_key(&animation_state) {
            panic!("{:?} was already in animation dictionary", animation_state);
        }

        // Use storm to load the image
        let loaded_texture = ctx.load_png(atlas);
        self.animations.insert(animation_state, loaded_texture);
    }

    //Returns a immutable reference to the underalying loaded atlas, which has been loaded by Storm
    pub fn get_atlas_for_animation(&self, animation_state: AnimationState) -> &Texture<RGBA8> {
        let current_animation = self.animations.get(&animation_state).unwrap();
        return &current_animation;
    }

    //Returns the subsection of an atlas used for rendering
    //Makes an assumption that all frames for all animations are of the same width
    //returns the section starting at frame_number * FRAME_WIDTH to frame_number * FRAME_WIDTH + FRAME_WIDTH
    pub fn get_atlas_subsection(& self, animation: AnimationState, frame_number: u32) -> TextureSection {
        let left = frame_number * FRAME_WIDTH;
        return self.animations.get(&animation).unwrap().subsection(left, left + FRAME_WIDTH, 0, FRAME_HEIGHT);
    }
}