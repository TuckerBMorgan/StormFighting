use serde::{Deserialize, Serialize};
use ggrs::GameInput;
use storm::math::AABB2D;
use storm::cgmath::*;
use super::{*};


#[derive(Serialize, Deserialize)]
pub struct Round {
    pub character_1: Character,
    pub character_2: Character,
    pub frame: i32,
    pub round_timer: SpriteTimer,
    pub round_done: bool,
    pub hit_stun_counter: usize

}

impl Round {
    pub fn advance(&mut self, inputs: Vec<GameInput>, collision_library: &CollisionLibrary) {
        
        self.frame += 1;
        if self.hit_stun_counter > 0 {
            self.hit_stun_counter -= 1;
            return;
        }
        self.character_1.tick(Input::from_game_input(inputs[0]));
        self.character_2.tick(Input::from_game_input(inputs[1]));

        self.round_timer.tick();

        let mut character_1_walk_box = self.character_1.get_walk_box();
        let mut character_2_walk_box = self.character_2.get_walk_box();
        //Then, if the character is moving we apply the desired change as a slide function
        //First character 1, then character 2
        //TODO: find out if order of these has gameplay implications
        // MAYBE have it be random?
        if self.character_1.character_velocity.x != 0.0 {
            if character_1_walk_box.slide(&self.character_1.character_velocity, &[character_2_walk_box]) {
                //Overlap. hmmmm
            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.character_1.character_position = character_1_walk_box.min - Vector2::new(131.0, 57.0);
        }

        
        if self.character_2.character_velocity.x != 0.0 {
            if character_2_walk_box.slide(&self.character_2.character_velocity, &[character_1_walk_box]) {
                //Overlap. hmmmm

            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.character_2.character_position = character_2_walk_box.min - Vector2::new(131.0, 57.0);
        }     
        if self.character_1.is_in_damageable_state() == false || self.character_2.is_in_damageable_state() == false {
            //TODO: handle invulnrability better, for now we are just gonna ignore certain states
            return;
        }
        

        let character_1_collision_key = self.character_1.get_collision_box_lookup_info();
        let current_aabbs_for_character_1 = collision_library.collision_info.get(&character_1_collision_key.0).unwrap().frame_collision.get(&character_1_collision_key.1).unwrap();

        let character_2_collision_key = self.character_2.get_collision_box_lookup_info();
        let current_aabbs_for_character_2 = collision_library.collision_info.get(&character_2_collision_key.0).unwrap().frame_collision.get(&character_2_collision_key.1).unwrap();

        //These two forloops for the same thing, just with a different character, they create new AABBS
        //Using the dimensions of the ones from the library, shifted by the characters position 
        let mut character_1_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_1 {
            let mut use_aabb = cb.aabb;
            if self.character_1.screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + self.character_1.character_position;
            let new_max = use_aabb.max + self.character_1.character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            character_1_position_corrected_aabbs.push((aabb, cb.box_type));
        }

        let mut character_2_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_2 {
            let mut use_aabb = cb.aabb;
            if self.character_2.screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + self.character_2.character_position;
            let new_max = use_aabb.max + self.character_2.character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            //Stick it into a vec for use later
            character_2_position_corrected_aabbs.push((aabb, cb.box_type));
        }

        //Get just the hurt boxes
        let character_1_hurt_boxes : Vec<_> = character_1_position_corrected_aabbs.iter().filter(|x|{
            return x.1 == CollisionBoxType::Hurt;
        }).collect();

        let character_2_hurt_boxes : Vec<_> = character_2_position_corrected_aabbs.iter().filter(|x|{
            return x.1 == CollisionBoxType::Hurt;
        }).collect();

        //For each characters hurt boxes, check them against the other characters total set of Hurt and Hit Boxes
        let mut collision_reports = vec![];
        for hurt_box in character_1_hurt_boxes {
            for aabb in character_2_position_corrected_aabbs.iter() {
                if hurt_box.0.intersects(&aabb.0) {
                    let collision_report = CollisionReport::new(hurt_box.1, aabb.1, CharacterNumber::Number1);
                    collision_reports.push(collision_report);
                }
            }
        }
        
        for hurt_box in character_2_hurt_boxes {
            for aabb in character_1_position_corrected_aabbs.iter() {
                if hurt_box.0.intersects(&aabb.0) {
                    let collision_report = CollisionReport::new(hurt_box.1, aabb.1, CharacterNumber::Number2);
                    collision_reports.push(collision_report);
                }
            }
        }

        //Parries are those overlaps that are Hurt or Hurt boxes
        let parries = collision_reports.iter().filter(|x|{
            return x.collider_type == CollisionBoxType::Hurt && x.collide_type == x.collider_type;
        });

        //Strikes are Hurt on Hit boxes
        let strikes = collision_reports.iter().filter(|x|{
            return x.collider_type == CollisionBoxType::Hurt && x.collide_type != x.collider_type;
        });


        //TODO: handle parries, idk, things do 
        for _par in parries {
            
        }

        //Preform strikes and assign damage
        for strike in strikes {
            match strike.collider_character {
                CharacterNumber::Number1 =>  {
                    self.character_2.do_damage(self.character_1.get_current_damage());
                    self.hit_stun_counter += 3;
                },
                CharacterNumber::Number2 => {
                    self.character_1.do_damage(self.character_2.get_current_damage());
                    self.hit_stun_counter += 3;
                }
            }
        }

        //If either player has died
        if self.character_1.health == 0 || self.character_2.health == 0 {
            self.round_done = true;
        }
        //Or we have just finished the game
        else if self.round_timer.finished() {
            self.round_done = true;
        }
    }
}

impl Default for Round{
    fn default() -> Round {
        //Build up the character by loading animations for each of the animation states
        let mut character_1 = Character::default();
        character_1.screen_side = ScreenSide::Right;
        character_1.character_position.x = (FRAME_WIDTH as f32) / 3.5;

        let mut character_2 = Character::default();
        character_2.screen_side = ScreenSide::Left;
        character_2.character_position.x = -(FRAME_WIDTH as f32) * 1.2;
        
        Round {
            character_1,
            character_2,
            frame: 0,
            round_timer: SpriteTimer::new(60 * 60),
            round_done: false,
            hit_stun_counter: 0
        }
    }
}