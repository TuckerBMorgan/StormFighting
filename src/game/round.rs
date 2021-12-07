use serde::{Deserialize, Serialize};
use ggrs::GameInput;
use storm::math::AABB2D;
use storm::cgmath::*;
use super::{*};


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

        let mut character_1_walk_box = self.character_1.get_walk_box();
        let mut character_2_walk_box = self.character_2.get_walk_box();
        //Then, if the character is moving we apply the desired change as a slide function
        //First character 1, then character 2
        //TODO: find out if order of these has gameplay implications
        // MAYBE have it be random?
        if self.character_1.character_state == CharacterState::ForwardRun || self.character_1.character_state == CharacterState::BackwardRun {

            if character_1_walk_box.slide(&self.character_1.character_velocity, &[character_2_walk_box]) {
                //Overlap. hmmmm
            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.character_1.character_position = character_1_walk_box.min - Vector2::new(131.0, 57.0);
        }

        
        if self.character_2.character_state == CharacterState::ForwardRun || self.character_2.character_state == CharacterState::BackwardRun {
            if character_2_walk_box.slide(&self.character_2.character_velocity, &[character_1_walk_box]) {
                //Overlap. hmmmm
            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.character_2.character_position = character_2_walk_box.min - Vector2::new(131.0, 57.0);
        }

        let character_1_collision_key = self.character_1.get_collision_box_lookup_info();
        let current_aabbs_for_character_1 = collision_library.collision_info.get(&character_1_collision_key.0).unwrap().frame_collision.get(&character_1_collision_key.1).unwrap();

        let character_2_collision_key = self.character_2.get_collision_box_lookup_info();
        let current_aabbs_for_character_2 = collision_library.collision_info.get(&character_2_collision_key.0).unwrap().frame_collision.get(&character_2_collision_key.1).unwrap();

        //These two forloops for the same thing, just with a different character, they create new AABBS
        //Using the dimensions of the ones from the library, shifted by the characters position 
        let mut character_1_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_1 {
            //Do the shift
            let new_min = cb.aabb.min + self.character_1.character_position;
            let new_max = cb.aabb.max + self.character_1.character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            //Stick it into a vec for use later
            character_1_position_corrected_aabbs.push((aabb, cb.box_type));
        }

        let mut character_2_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_2 {
            //Do the shift
            let new_min = cb.aabb.min + self.character_2.character_position;
            let new_max = cb.aabb.max + self.character_2.character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            //Stick it into a vec for use later
            character_2_position_corrected_aabbs.push((aabb, cb.box_type));
        }

        let character_1_hurt_boxes : Vec<_> = character_1_position_corrected_aabbs.iter().filter(|x|{
            return x.1 == CollisionBoxType::Hurt;
        }).collect();

        let character_2_hurt_boxes : Vec<_> = character_2_position_corrected_aabbs.iter().filter(|x|{
            return x.1 == CollisionBoxType::Hurt;
        }).collect();

        let mut collision_reports = vec![];
        for hurt_box in character_1_hurt_boxes {
            for aabb in character_2_position_corrected_aabbs.iter() {
                if hurt_box.0.intersects(&aabb.0) {
                    let collision_report = CollisionReport::new(hurt_box.1, aabb.1);
                    collision_reports.push(collision_report);
                }
            }
        }

        
        for hurt_box in character_2_hurt_boxes {
            for aabb in character_1_position_corrected_aabbs.iter() {
                if hurt_box.0.intersects(&aabb.0) {
                    let collision_report = CollisionReport::new(hurt_box.1, aabb.1);
                    collision_reports.push(collision_report);
                }
            }
        }

        let parries = collision_reports.iter().filter(|x|{
                return x.collider_type == CollisionBoxType::Hurt && x.collide_type == x.collider_type;
            }
        );

        let strikes = collision_reports.iter().filter(|x|{
            return x.collider_type == CollisionBoxType::Hurt && x.collide_type != x.collider_type;
            }
        );

        for par in parries {

        }

        for strike in strikes {
            
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