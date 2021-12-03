use serde::{Deserialize, Serialize};
use ggrs::GameInput;
use storm::math::AABB2D;

use super::{Character, Input, ScreenSide, AnimationState, AnimationConfig, CharacterState, CollisionLibrary};


#[derive(Serialize, Deserialize)]
pub struct Round {
    pub character_1: Character,
    pub character_2: Character,
    pub frame: i32

}

impl Round {

    pub fn advance(&mut self, inputs: Vec<GameInput>, collision_library: &CollisionLibrary) {
        self.frame += 1;
        self.character_1.tick(Input::from_game_input(inputs[0]));
        self.character_2.tick(Input::from_game_input(inputs[1]));

        let character_1_collision_key = self.character_1.get_collision_box_lookup_info();
        let current_aabbs_for_character_1 = collision_library.collision_info.get(&character_1_collision_key.0).unwrap().frame_collision.get(&character_1_collision_key.1).unwrap();

        let character_2_collision_key = self.character_2.get_collision_box_lookup_info();
        let current_aabbs_for_character_2 = collision_library.collision_info.get(&character_2_collision_key.0).unwrap().frame_collision.get(&character_2_collision_key.1).unwrap();

        //TODO: COMMENT THE FUCK OUT OF THIS NONSENSE

        let mut character_1_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_1 {
            let new_min = cb.aabb.min + self.character_1.character_position;
            let new_max = cb.aabb.max + self.character_1.character_position;
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            character_1_position_corrected_aabbs.push(aabb);
        }

        let mut character_2_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_2 {
            let new_min = cb.aabb.min + self.character_2.character_position;
            let new_max = cb.aabb.max + self.character_2.character_position;
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            character_2_position_corrected_aabbs.push(aabb);
        }

        if self.character_1.character_state == CharacterState::ForwardRun || self.character_1.character_state == CharacterState::BackwardRun {
            let mut body_aabb = character_1_position_corrected_aabbs[0];
            if body_aabb.slide(&self.character_1.character_velocity, &character_2_position_corrected_aabbs) {
                //Overlap. hmmmm
            }
            self.character_1.character_position = body_aabb.min;
            self.character_1.character_position.x -= 150.0;
            self.character_1.character_position.y -= 53.0;

        }

        
        if self.character_2.character_state == CharacterState::ForwardRun || self.character_2.character_state == CharacterState::BackwardRun {
            let mut body_aabb = character_2_position_corrected_aabbs[0];
            if body_aabb.slide(&self.character_2.character_velocity, &character_1_position_corrected_aabbs) {
                //Overlap. hmmmm
            }

            self.character_2.character_position = body_aabb.min;
            self.character_2.character_position.x -= 150.0;
            self.character_2.character_position.y -= 53.0;
        }


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