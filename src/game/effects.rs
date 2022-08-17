use crate::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,  Clone)]
pub enum EffectKind {
    Low,
    Medium,
    High
}

// A Tracker of an effect in the world, how long it will last, and what level it is
#[derive(Serialize, Deserialize,  Clone)]
pub struct Effect {
    pub sprite_timer: SpriteTimer,
    pub number_of_frames: usize,
    pub current_frame: usize,
    pub is_use: bool,
    pub effect_kind: EffectKind,
    pub position_x: f32,
    pub position_y: f32,
    pub screen_side: ScreenSide
}


impl Effect {
    pub fn new(frame_duration: usize, number_of_frames: usize, effect_kind: EffectKind, position_x: f32, position_y: f32, screen_side: ScreenSide) -> Effect {
        Effect {
            sprite_timer: SpriteTimer::new(frame_duration as u32),
            number_of_frames,
            current_frame: 0,
            is_use: true,
            effect_kind,
            position_x,
            position_y,
            screen_side
        }
    }

    pub fn advance(&mut self) {
        self.sprite_timer.tick();
        if self.sprite_timer.finished() {
            self.current_frame += 1;
            if self.current_frame == self.number_of_frames {
                self.is_use = false;
            }            
        }
    }
}

