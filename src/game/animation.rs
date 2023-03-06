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
//pub static BUTTON: &[u8] = include_bytes!("../../resources/button.png");

pub const FRAME_HEIGHT: u32 =  178;
pub const FRAME_WIDTH: u32 =  290;

pub const EFFECT_FRAME_WIDTH: u32 = 640;



#[macro_export]
macro_rules! animation_state_enum {
    ( $( $x:ident ),* ) => {
         #[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
          pub enum AnimationState {
            $(
                $x,
            )*
          }

          impl AnimationState {
            pub fn from_string(value:&String) -> AnimationState {
              $(
                if value == stringify!($x) {
                  return AnimationState::$x;
                }
              )*
                      panic!("{:?} is an unknow animation state", value);
            }

            pub fn to_string(&self) -> String {
              match *self {
                $(
                AnimationState::$x => {
                  return String::from(stringify!($x));
                }
                )*
              }
            }
          }
    };
}


animation_state_enum!(    Idle,
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
    LightCrouchKick,
    MediumCrouchKick,
    HeavyCrouchKick,
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
    ForwardJump,
    LightJumpingKick,
    JumpingLightPunch,
    Dizzie);

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
//        let loaded_texture = Texture::from_png(ctx, atlas, TextureFiltering::none());
        let width = [atlas[0], atlas[1], atlas[2], atlas[3]];
        let width = u32::from_le_bytes(width);
        let height = [atlas[4], atlas[5], atlas[6], atlas[7]];
        let height = u32::from_le_bytes(height);
        let loaded_texture = Texture::from_image(ctx, &storm::image::Image::from_vec(atlas[8..].to_vec(), width, height), TextureFiltering::none());
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


#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EffectAnimationState {
    LightHit
}

pub struct EffectAnimationTextureLibrary {
    pub animations: HashMap<EffectAnimationState, Texture>,
}

impl EffectAnimationTextureLibrary {
    pub fn new() -> EffectAnimationTextureLibrary {
    EffectAnimationTextureLibrary {
            animations: HashMap::new()
        }
    }

    // Given atlas(a u8 representation of the image we want) and the animation state we want
    // Build a mapping between the two so that we can look it up later
    pub fn load_animation(&mut self, atlas: &[u8], animation_state: EffectAnimationState, ctx: &mut Context<FighthingApp>) {
        if self.animations.contains_key(&animation_state) {
            panic!("{:?} was already in animation dictionary", animation_state);
        }

        // Use storm to load the image
        let loaded_texture = Texture::from_png(ctx, atlas, TextureFiltering::none());
        self.animations.insert(animation_state, loaded_texture);
    }

    //Returns a immutable reference to the underalying loaded atlas, which has been loaded by Storm
    pub fn get_atlas_for_animation(&self, animation_state: EffectAnimationState) -> Texture {
        let current_animation = self.animations.get(&animation_state).unwrap();
        return current_animation.clone();
    }

    //Returns the subsection of an atlas used for rendering
    //Makes an assumption that all frames for all animations are of the same width
    //returns the section starting at frame_number * FRAME_WIDTH to frame_number * FRAME_WIDTH + FRAME_WIDTH
    pub fn get_atlas_subsection(& self, animation: EffectAnimationState, frame_number: u32) -> TextureSection {
        let left = frame_number * FRAME_WIDTH;
        return self.animations.get(&animation).unwrap().subsection(left, left + FRAME_WIDTH, 0, FRAME_HEIGHT);
    }
}