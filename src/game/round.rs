use serde::{Deserialize, Serialize};
use ggrs::GameInput;
use storm::math::AABB2D;
use storm::cgmath::*;
use super::*;


#[derive(Serialize, Deserialize)]
pub struct Round {
    pub characters: Vec<Character>,
    pub frame: i32,
    pub round_timer: SpriteTimer,
    pub round_done: bool,
    pub hit_stun_counter: usize,
    pub projectiles: Vec<Projectile>

}

impl Round {
    pub fn advance(&mut self, inputs: Vec<GameInput>, game_config: &mut GameConfig) {
        
        self.frame += 1;
        if self.hit_stun_counter > 0 {
            self.hit_stun_counter -= 1;
            return;
        }
        self.character_tick(0, Input::from_game_input(inputs[0]), game_config);
        self.character_tick(1, Input::from_game_input(inputs[1]), game_config);

        self.round_timer.tick();

        let mut character_1_walk_box = self.characters[0].get_walk_box();
        let mut character_2_walk_box = self.characters[1].get_walk_box();
        //Then, if the character is moving we apply the desired change as a slide function
        //First character 1, then character 2
        //TODO: find out if order of these has gameplay implications
        // MAYBE have it be random?
        if self.characters[0].character_velocity.x != 0.0 {
            if character_1_walk_box.slide(&self.characters[0].character_velocity, &[character_2_walk_box]) {
                //Overlap. hmmmm
            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.characters[0].character_position = character_1_walk_box.min - Vector2::new(131.0, 57.0);
        }

        
        if self.characters[1].character_velocity.x != 0.0 {
            if character_2_walk_box.slide(&self.characters[1].character_velocity, &[character_1_walk_box]) {
                //Overlap. hmmmm

            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.characters[1].character_position = character_2_walk_box.min - Vector2::new(131.0, 57.0);
        }     

        for projectile in self.projectiles.iter_mut() {
            projectile.tick();
        }

        if self.characters[0].is_in_damageable_state() == false || self.characters[1].is_in_damageable_state() == false {
            //TODO: handle invulnrability better, for now we are just gonna ignore certain states
            return;
        }

        let character_1_collision_key = self.characters[0].get_collision_box_lookup_info();
        let current_aabbs_for_character_1 = game_config.collision_library.collision_info.get(&character_1_collision_key.0).unwrap().frame_collision.get(&character_1_collision_key.1).unwrap();

        let character_2_collision_key = self.characters[1].get_collision_box_lookup_info();
        let current_aabbs_for_character_2 = game_config.collision_library.collision_info.get(&character_2_collision_key.0).unwrap().frame_collision.get(&character_2_collision_key.1).unwrap();

        //These two forloops for the same thing, just with a different character, they create new AABBS
        //Using the dimensions of the ones from the library, shifted by the characters position 
        let mut character_1_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_1 {
            let mut use_aabb = cb.aabb;
            if self.characters[0].screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + self.characters[0].character_position;
            let new_max = use_aabb.max + self.characters[0].character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            character_1_position_corrected_aabbs.push((aabb, cb.box_type));
        }

        let mut character_2_position_corrected_aabbs = vec![];
        for cb in current_aabbs_for_character_2 {
            let mut use_aabb = cb.aabb;
            if self.characters[1].screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + self.characters[1].character_position;
            let new_max = use_aabb.max + self.characters[1].character_position;
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
                    let damage_amount = self.characters[0].get_current_damage();
                    self.do_damage_to_character(1, damage_amount, game_config);
                    self.hit_stun_counter += 3;
                },
                CharacterNumber::Number2 => {
                    let damage_amount = self.characters[1].get_current_damage();
                    self.do_damage_to_character(0, damage_amount, game_config);
                    self.hit_stun_counter += 3;
                }
            }
        }
        /*
        let fireball_collision = &collision_library.fireball_collision;
        for projectile in self.projectiles.iter() {
            let mut use_aabb = fireball_collision.frame_collision.get(&0).unwrap()[0].aabb;
            if projectile.screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + self.character_1.character_position;
            let new_max = use_aabb.max + self.character_1.character_position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            character_1_position_corrected_aabbs.push((aabb, CollisionBoxType::Hurt));
        }
        */


        //If either player has died
        if self.characters[0].health == 0 || self.characters[1].health == 0 {
            self.round_done = true;
        }
        //Or we have just finished the game
        else if self.round_timer.finished() {
            self.round_done = true;
        }
    }

    pub fn character_tick(&mut self, character_index: usize, frame_input: Input, game_config: &mut GameConfig) {
        let frame_input = ScreenSideAdjustedInput::new(&frame_input, self.characters[character_index].screen_side);

        let character_action = self.characters[character_index].process_new_input(frame_input.clone(), &mut game_config.combo_library);
        //We want an hierarcy of input to handle people button mashing
        //A character should generally be Attacking Over Moving Over Doing Nothing
        if self.characters[character_index].can_attack() {
            if character_action == CharacterAction::LightAttack {
                self.characters[character_index].set_character_state(CharacterState::LightAttack, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::MediumAttack {
                self.characters[character_index].set_character_state(CharacterState::MediumAttack, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::HeavyAttack {
                self.characters[character_index].set_character_state(CharacterState::HeavyAttack, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::LightKick {
                self.characters[character_index].set_character_state(CharacterState::LightKick, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::MediumKick {
                self.characters[character_index].set_character_state(CharacterState::MediumKick, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::HeavyKick {
                self.characters[character_index].set_character_state(CharacterState::HeavyKick, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::Special1 {
                let mut velocity = Vector2::new(-10.0, 0.0);
                if self.characters[character_index].screen_side == ScreenSide::Left {
                    velocity.x = 10.0;
                }
                
                let fireball = Projectile::new(self.characters[character_index].character_position + Vector2::new(-10.0, -75.0), velocity, self.characters[character_index].screen_side);
                self.projectiles.push(fireball);
                self.characters[character_index].set_character_state(CharacterState::Special1, &game_config.animation_for_character_state_library);
            }
        }
        //If we are in the normal crouched animation, Idle + IsCrouched, and we are no longer holding the down key
        //Stand the character up
        //TODO: add in a "standing_up" animation state and animation
        if self.characters[character_index].character_state == CharacterState::Idle && self.characters[character_index].is_crouched && character_action != CharacterAction::Crouch {
            self.characters[character_index].is_crouched = false;
            self.characters[character_index].set_character_state(CharacterState::Idle, &game_config.animation_for_character_state_library);
        }

        if self.characters[character_index].character_state == CharacterState::Idle || self.characters[character_index].character_state == CharacterState::BackwardRun || self.characters[character_index].character_state == CharacterState::ForwardRun {
            if character_action == CharacterAction::DashForward{
                self.characters[character_index].set_character_state(CharacterState::ForwardDash, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::DashBackward {
                self.characters[character_index].set_character_state(CharacterState::BackwardDash, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::MoveForward {
                self.characters[character_index].set_character_state(CharacterState::ForwardRun, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::MoveBackward {
                self.characters[character_index].set_character_state(CharacterState::BackwardRun, &game_config.animation_for_character_state_library);
            }
            else if character_action == CharacterAction::Crouch {
                if self.characters[character_index].is_crouched == false {
                    self.characters[character_index].is_crouched = true;
                    self.characters[character_index].set_character_state(CharacterState::Crouching, &game_config.animation_for_character_state_library);
                }
            }
        }

        if self.characters[character_index].character_state == CharacterState::ForwardRun || self.characters[character_index].character_state == CharacterState::BackwardRun {
            if character_action == CharacterAction::None {
                self.characters[character_index].set_character_state(CharacterState::Idle, &game_config.animation_for_character_state_library);
            }
        }

        //Then tick the animations to see if we have finished any and we need to be in a new state
        let foo = self.characters[character_index].animation_state.clone();
        let mut current_animation = self.characters[character_index].animation_configs.get_mut(&foo).unwrap();
        current_animation.sprite_timer.tick();
        if current_animation.sprite_timer.finished() {
            current_animation.sprite_timer.reset();
            current_animation.current_frame += 1;
            //If we have finished the animation move the character into the
            //next state, be that loop(like idle or run)
            //or a steady state like Attack -> Idle
            if current_animation.is_done() {
                current_animation.reset();
                let new_state = {
                    self.characters[character_index].finished_animation_whats_next()
                };
                self.characters[character_index].set_character_state(new_state, &game_config.animation_for_character_state_library);
            }
        }
        
        //Lasting state doing state based actions like, moving
        if self.characters[character_index].character_state == CharacterState::ForwardRun {
            self.characters[character_index].character_velocity.x = -(CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction());
        }
        else if self.characters[character_index].character_state == CharacterState::BackwardRun {
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction();
        }
        else if self.characters[character_index].character_state == CharacterState::ForwardDash {
            self.characters[character_index].character_velocity.x = -(CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction() * 2.0);
        }
        else if self.characters[character_index].character_state == CharacterState::BackwardDash {
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction() * 2.0;
        }
        else if self.characters[character_index].character_state == CharacterState::LightHitRecovery {
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction();
        }
        else {
            self.characters[character_index].character_velocity.x = 0.0;
        }
    }

    pub fn do_damage_to_character(&mut self, character_index: usize, amount: u32, game_config: &mut GameConfig) {
        match self.characters[character_index].character_state {
            CharacterState::Blocking => {
                if self.characters[character_index].health <= (amount/10) {
                    self.characters[character_index].health = 0;
                }
                else {
                    self.characters[character_index].health -= amount / 10;
                }
                self.characters[character_index].set_character_state(CharacterState::Blocking, &game_config.animation_for_character_state_library);
            }
            _ => {
                if self.characters[character_index].health <= amount {
                    self.characters[character_index].health = 0;
                }
                else {
                    self.characters[character_index].health -= amount;
                }
                self.characters[character_index].set_character_state(CharacterState::LightHitRecovery, &game_config.animation_for_character_state_library);
            }
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
            characters: vec![character_1, character_2],
            frame: 0,
            round_timer: SpriteTimer::new(60 * 60),
            round_done: false,
            hit_stun_counter: 0,
            projectiles: vec![]
        }
    }
}