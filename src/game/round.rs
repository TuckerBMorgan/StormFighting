use serde::{Deserialize, Serialize};
use ggrs::GameInput;

use super::{Character, Input, ScreenSide, AnimationState, AnimationConfig, CharacterState, CollisionLibrary};


#[derive(Serialize, Deserialize)]
pub struct Round {
    pub character_1: Character,
    pub character_2: Character,
    pub frame: i32

}

impl Round {
    pub fn new(character_1: Character, character_2: Character) -> Round {
        Round {
            character_1,
            character_2,
            frame: 0
        }
    }

    pub fn advance(&mut self, inputs: Vec<GameInput>, collision_library: &CollisionLibrary) {
        self.frame += 1;
        self.character_1.tick(Input::from_game_input(inputs[0]));
        self.character_2.tick(Input::from_game_input(inputs[1]));

        let character_1_collision_key = self.character_1.get_collision_box_lookup_info();
        let current_aabbs_for_character_1 = collision_library.collision_info.get(&character_1_collision_key.0).unwrap().frame_collision.get(&character_1_collision_key.1).unwrap();

        let character_2_collision_key = self.character_2.get_collision_box_lookup_info();
        let current_aabbs_for_character_2 = collision_library.collision_info.get(&character_2_collision_key.0).unwrap().frame_collision.get(&character_2_collision_key.1).unwrap();

        let mut character_1_aabb = self.character_1.get_current_aabb();
        let mut character_2_aabb = self.character_2.get_current_aabb();
        if self.character_1.character_state == CharacterState::ForwardRun || self.character_1.character_state == CharacterState::BackwardRun {      
            if character_1_aabb.slide(&self.character_1.character_velocity, &[character_2_aabb]) {
                //Overlap. hmmmm

            }
        }
        /*
        if self.character_2.character_state == CharacterState::ForwardRun || self.character_2.character_state == CharacterState::BackwardRun {
            if character_2_aabb.slide(&self.character_2.character_velocity, &[character_1_aabb]) {
                //Overlap. hmmmm
            }
        }
        */
        self.character_1.character_position = character_1_aabb.min;
        self.character_1.character_position.x -= 150.0;
        self.character_1.character_position.y -= 53.0;

        self.character_2.character_position = character_2_aabb.min;
        self.character_2.character_position.x -= 150.0;
        self.character_2.character_position.y -= 53.0;
    }
}


impl Default for Round{
    fn default() -> Round {
            //Build up the character by loading animations for each of the animation states
        let mut character_1 = Character::new(ScreenSide::Right);
        character_1.load_animation_config(AnimationState::Idle, AnimationConfig::new(10, 4));
        character_1.load_animation_config(AnimationState::ForwardRun, AnimationConfig::new(12, 4));
        character_1.load_animation_config(AnimationState::LightAttack, AnimationConfig::new(5, 4));
        character_1.load_animation_config(AnimationState::BackwardRun, AnimationConfig::new(10, 4));

        let mut character_2 = Character::new(ScreenSide::Left);
        character_2.load_animation_config(AnimationState::Idle, AnimationConfig::new(10, 4));
        character_2.load_animation_config(AnimationState::ForwardRun, AnimationConfig::new(12, 4));
        character_2.load_animation_config(AnimationState::LightAttack, AnimationConfig::new(5, 4));
        character_2.load_animation_config(AnimationState::BackwardRun, AnimationConfig::new(10, 4));
        Round {
            character_1,
            character_2,
            frame: 0
        }
    }
}