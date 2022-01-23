use serde::{Deserialize, Serialize};

use storm::cgmath::Vector2;

use super::{ScreenSide, AnimationConfig};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Projectile {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub screen_side: ScreenSide,
    pub timer: AnimationConfig,
    pub team: usize
}

impl Projectile {
    pub fn new(    
        position: Vector2<f32>,
        velocity: Vector2<f32>,
        screen_side: ScreenSide,
        team: usize
    ) -> Projectile {
        Projectile {
            position,
            velocity,
            screen_side,
            timer: AnimationConfig::new(20, 4),
            team
        }
    }

    pub fn tick(&mut self) {
        self.position += self.velocity;
        if self.timer.sprite_timer.finished() {
            self.timer.sprite_timer.reset();
            self.timer.current_frame += 1;
            //If we have finished the animation move the character into the
            //next state, be that loop(like idle or run)
            //or a steady state like Attack -> Idle
            if self.timer.is_done() {
                self.timer.reset();
            }
        }
    }
}