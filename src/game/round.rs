use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use ggrs::GameInput;
use storm::math::AABB2D;
use storm::cgmath::*;


use super::*;

const MAX_PLAYER_DISTANCE : f32 = FRAME_WIDTH as f32;

#[derive(Serialize, Deserialize,  Clone)]
pub struct Round {
    pub characters: Vec<Character>,
    pub frame: i32,
    pub round_timer: SpriteTimer,
    pub round_done: bool,
    pub hit_stun_counter: usize,
    pub projectiles: Vec<Projectile>,
    pub reset_round_timer: SpriteTimer
}

impl Round {
    pub fn advance(&mut self, inputs: Vec<GameInput>, game_config: &mut GameConfig) {
        
        self.frame += 1;
        if self.hit_stun_counter > 0 {
            self.hit_stun_counter -= 1;
            return;
        }

        if self.characters[0].character_position.x > self.characters[1].character_position.x {
            self.characters[0].screen_side = ScreenSide::Right;
            self.characters[1].screen_side = ScreenSide::Left;
        }
        else {
            self.characters[0].screen_side = ScreenSide::Left;
            self.characters[1].screen_side = ScreenSide::Right;
        }

        
        self.character_tick(0, Input::from_game_input(inputs[0].clone()), game_config);
        self.character_tick(1, Input::from_game_input(inputs[1].clone()), game_config);

        for projectile in self.projectiles.iter_mut() {
            projectile.tick();
        }

        //Leave early if we are wating for a round to fully end for the reset
        if self.round_done {
            let (won, lost) = self.who_won_who_lost();
            self.characters[won].set_character_state(CharacterState::Won, &game_config);
            self.characters[won].done = true;
            self.characters[lost].set_character_state(CharacterState::Lost, &game_config);
            self.characters[lost].done = true;
            self.reset_round_timer.tick();
            return;
        }

        self.round_timer.tick();

        let mut character_1_walk_box = self.characters[0].get_walk_box();
        let mut character_2_walk_box = self.characters[1].get_walk_box();

        //TODO: these two functions are very fragile, would like to refactor them
        //into a single funciton on character
        if self.characters[0].character_velocity.x != 0.0 || self.characters[0].character_velocity.y != 0.0 {
            let mut reshift = Vector2::new(0.0, 0.0);
            if self.characters[1].character_state == CharacterState::Jump {
                if character_1_walk_box.slide(&self.characters[0].character_velocity, &[]) {
                    //Overlap. hmmmm
                    reshift.x = CHARACTER_X_SPEED * 1.1 * self.characters[0].screen_side.direction() * -1.0;
                }
            }
            else { 
                if character_1_walk_box.slide(&self.characters[0].character_velocity, &[character_2_walk_box]) {
                    //Overlap. hmmmm

                }
            }

            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.characters[0].character_position = character_1_walk_box.min - Vector2::new(131.0, 57.0) + reshift;

            //Keep the character in the arena, and close enough to the other playear
            if self.characters[0].character_position.x  < 0.0  {
                self.characters[0].character_position.x = 0.0;
            }
            else if self.characters[0].character_position.x + FRAME_WIDTH as f32 >= 896.0 { 
                self.characters[0].character_position.x = 896.0 - FRAME_WIDTH as f32;
            }
            else if f32::abs(self.characters[0].character_position.x - self.characters[1].character_position.x) > MAX_PLAYER_DISTANCE {
                //TODO: Handle screen sides when we add in jumping
                self.characters[0].character_position.x = self.characters[1].character_position.x + MAX_PLAYER_DISTANCE;
            }
        }


        if self.characters[1].character_velocity.x != 0.0 || self.characters[1].character_velocity.y != 0.0 {
            let mut reshift = Vector2::new(0.0, 0.0);
            if self.characters[0].character_state == CharacterState::Jump {
                if character_2_walk_box.slide(&self.characters[1].character_velocity, &[]) {
                    //Overlap. hmmmm
                    reshift.x = CHARACTER_X_SPEED * 1.1 * self.characters[1].screen_side.direction() * -1.0;
                }
            }
            else { 
                if character_2_walk_box.slide(&self.characters[1].character_velocity, &[character_1_walk_box]) {
                    //Overlap. hmmmm

                }
            }
            //We need to remove the offset that we build in from the initial unshifted AABBS
            //This will give us the characters new position
            self.characters[1].character_position = character_2_walk_box.min - Vector2::new(131.0, 57.0) + reshift;

            if self.characters[1].character_position.x  < 0.0  {
                self.characters[1].character_position.x = 0.0;
            }
            else if self.characters[1].character_position.x + FRAME_WIDTH as f32 >= 896.0 { 
                self.characters[1].character_position.x = 896.0 - FRAME_WIDTH as f32;
            }
            else if f32::abs(self.characters[0].character_position.x - self.characters[1].character_position.x) > MAX_PLAYER_DISTANCE {
                //TODO: Handle screen sides when we add in jumping
                //Maybe add in jumping tonight???__??
                self.characters[1].character_position.x = self.characters[0].character_position.x - MAX_PLAYER_DISTANCE;
            }

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
            return x.1 == CollisionBoxType::Hurt || return x.1 == CollisionBoxType::Parry;
        }).collect();

        let character_2_hurt_boxes : Vec<_> = character_2_position_corrected_aabbs.iter().filter(|x|{
            return x.1 == CollisionBoxType::Hurt || return x.1 == CollisionBoxType::Parry;
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


        //If a parry box hit a 
        let parries = collision_reports.iter().filter(|x|{
            return x.collider_type == CollisionBoxType::Parry && x.collide_type == CollisionBoxType::Hurt;
        });

        //Strikes are Hurt on Hit boxes
        let strikes = collision_reports.iter().filter(|x|{
            return x.collider_type == CollisionBoxType::Hurt && x.collide_type != x.collider_type;
        });


        //TODO: handle parries, idk, things do 
        for par in parries {
            match par.collider_character {
                CharacterNumber::Number1 =>  {
                    let damage_amount = 5;
                    self.do_damage_to_character(1, damage_amount, true, game_config);
                    self.hit_stun_counter += 6;
                },
                CharacterNumber::Number2 => {
                    let damage_amount = 5;
                    self.do_damage_to_character(0, damage_amount, true, game_config);
                    self.hit_stun_counter += 6;
                }
            }
        }

        //Preform strikes and assign damage
        for strike in strikes {
            match strike.collider_character {
                CharacterNumber::Number1 =>  {
                    let damage_amount = self.characters[0].get_current_damage();
                    self.do_damage_to_character(1, damage_amount, false, game_config);
                    self.hit_stun_counter += 3;
                },
                CharacterNumber::Number2 => {
                    let damage_amount = self.characters[1].get_current_damage();
                    self.do_damage_to_character(0, damage_amount, false, game_config);
                    self.hit_stun_counter += 3;
                }
            }
        }
        
        let fireball_collision = &game_config.collision_library.fireball_collision;
        let mut projectile_position_corrected_aabbs = vec![];
        for projectile in self.projectiles.iter() {
            let mut use_aabb = fireball_collision.frame_collision.get(&0).unwrap()[0].aabb;
            if projectile.screen_side == ScreenSide::Left {
                use_aabb = use_aabb.reflect((FRAME_WIDTH / 2) as usize);
             }
            //Do the shift
            let new_min = use_aabb.min + projectile.position;
            let new_max = use_aabb.max + projectile.position;
            //Init the new AABB
            let aabb = AABB2D::new(new_min.x, new_min.y, new_max.x, new_max.y);
            projectile_position_corrected_aabbs.push((aabb, CollisionBoxType::Hurt, projectile.team));
        }
        
        collision_reports.clear();
        for (projectile_aab, box_type, team) in projectile_position_corrected_aabbs {
            if team == 0 {
                for aabb in character_2_position_corrected_aabbs.iter() {
                    if projectile_aab.intersects(&aabb.0) {
                        let collision_report = CollisionReport::new(box_type, aabb.1, CharacterNumber::Number1);
                        collision_reports.push(collision_report);
                    }
                }
            }
            else {
                for aabb in character_1_position_corrected_aabbs.iter() {
                    if projectile_aab.intersects(&aabb.0) {
                        let collision_report = CollisionReport::new(box_type, aabb.1, CharacterNumber::Number2);
                        collision_reports.push(collision_report);
                    }
                }
            }
        }

        if collision_reports.len() > 0 {
            self.projectiles.clear();
            for strike in collision_reports {
                match strike.collider_character {
                    CharacterNumber::Number1 =>  {
                        self.do_damage_to_character(1, 5, false, game_config);
                        self.hit_stun_counter += HITSTUN_AMOUNT;
                    },
                    CharacterNumber::Number2 => {
                        self.do_damage_to_character(0, 5, false, game_config);
                        self.hit_stun_counter += HITSTUN_AMOUNT;
                    }
                }
            }
        }

//        self.projectiles.retain(|x|x.position.x > -2000.0);

        //If either player has died
        if self.characters[0].health == 0 || self.characters[1].health == 0 {
            self.round_done = true;
        }
        //Or we have just finished the game
        else if self.round_timer.finished() {
            self.round_done = true;
        }

    }

    pub fn who_won_who_lost(&self) -> (usize, usize) {
        if self.characters[0].health > self.characters[1].health {
            return (0, 1);
        }
        return (1, 0);
    }

    pub fn character_tick(&mut self, character_index: usize, frame_input: Input, game_config: &mut GameConfig) {

        //Then tick the animations to see if we have finished any and we need to be in a new state

        self.characters[character_index].current_animation.sprite_timer.tick();
        if self.characters[character_index].current_animation.sprite_timer.finished() {
            self.characters[character_index].current_animation.current_frame += 1;
            self.characters[character_index].current_animation.sprite_timer.reset();


            //If we have finished the animation move the character into the
            //next state, be that loop(like idle or run)
            //or a steady state like Attack -> Idle

            if self.characters[character_index].current_animation.is_done() {
                self.characters[character_index].current_animation.reset();
                let new_state = {
                    self.characters[character_index].finished_animation_whats_next()
                };
                self.characters[character_index].set_character_state(new_state, &game_config);
            }
        }

        let frame_input = ScreenSideAdjustedInput::new(&frame_input, self.characters[character_index].screen_side);

        let character_action = self.characters[character_index].process_new_input(frame_input.clone(), &mut game_config.combo_library);
        //We want an hierarcy of input to handle people button mashing
        //A character should generally be Attacking Over Moving Over Doing Nothing

        //TODO: build some form of map for thios
        if self.characters[character_index].can_attack() {
            if character_action == CharacterAction::LightAttack {
                self.characters[character_index].set_character_state(CharacterState::LightAttack, &game_config);
            }
            else if character_action == CharacterAction::MediumAttack {
                self.characters[character_index].set_character_state(CharacterState::MediumAttack, &game_config);
            }
            else if character_action == CharacterAction::HeavyAttack {
                self.characters[character_index].set_character_state(CharacterState::HeavyAttack, &game_config);
            }
            else if character_action == CharacterAction::LightKick {
                self.characters[character_index].set_character_state(CharacterState::LightKick, &game_config);
            }
            else if character_action == CharacterAction::MediumKick {
                self.characters[character_index].set_character_state(CharacterState::MediumKick, &game_config);
            }
            else if character_action == CharacterAction::HeavyKick {
                self.characters[character_index].set_character_state(CharacterState::HeavyKick, &game_config);
            }
            else if character_action == CharacterAction::Special1 {
                self.characters[character_index].set_character_state(CharacterState::Special1, &game_config);
            }
            else if character_action == CharacterAction::Parry {
                self.characters[character_index].set_character_state(CharacterState::Parry, &game_config);
            }
        }

        if self.characters[character_index].character_state == CharacterState::Special1 {
            //TODO: make this, idk, something better, the fact that I just need to memorize what this
            //index is is BAD
            if self.characters[character_index].current_animation.current_frame == 6 
                && self.characters[character_index].current_animation.sprite_timer.current_frame == 0 {
                let mut velocity = Vector2::new(-10.0, 0.0);
                let start_offset;
                if self.characters[character_index].screen_side == ScreenSide::Left {
                    velocity.x = 10.0;
                    start_offset = Vector2::new(0.0, 0.0);
                }
                else {
                    start_offset = Vector2::new(0.0, 0.0);
                }
                
                let fireball = Projectile::new(self.characters[character_index].character_position + start_offset, velocity, self.characters[character_index].screen_side, character_index);
                self.projectiles.push(fireball);
            }
        }
        //If we are in the normal crouched animation, Idle + IsCrouched, and we are no longer holding the down key
        //Stand the character up
        //TODO: add in a "standing_up" animation state and animation
        if self.characters[character_index].character_state == CharacterState::Idle && self.characters[character_index].is_crouched && character_action != CharacterAction::Crouch {
            self.characters[character_index].is_crouched = false;
            self.characters[character_index].set_character_state(CharacterState::Idle, &game_config);
        }

        if self.characters[character_index].character_state == CharacterState::Idle || self.characters[character_index].character_state == CharacterState::BackwardRun || self.characters[character_index].character_state == CharacterState::ForwardRun {
            if character_action == CharacterAction::DashForward{
                self.characters[character_index].set_character_state(CharacterState::ForwardDash, &game_config);
            }
            else if character_action == CharacterAction::DashBackward {
                self.characters[character_index].set_character_state(CharacterState::BackwardDash, &game_config);
            }
            else if character_action == CharacterAction::MoveForward && self.characters[character_index].character_state != CharacterState::ForwardRun {
                self.characters[character_index].set_character_state(CharacterState::ForwardRun, &game_config);
            }
            else if character_action == CharacterAction::MoveBackward && self.characters[character_index].character_state != CharacterState::BackwardRun {
                self.characters[character_index].set_character_state(CharacterState::BackwardRun, &game_config);
            }
            else if character_action == CharacterAction::Crouch {
                if self.characters[character_index].is_crouched == false {
                    self.characters[character_index].is_crouched = true;
                    self.characters[character_index].set_character_state(CharacterState::Crouching, &game_config);
                }
            }
            else if character_action == CharacterAction::Jump {
                self.characters[character_index].set_character_state(CharacterState::Jump, &game_config);
            }
        }

        if self.characters[character_index].character_state == CharacterState::ForwardRun || self.characters[character_index].character_state == CharacterState::BackwardRun {
            if character_action == CharacterAction::None {
                self.characters[character_index].set_character_state(CharacterState::Idle, &game_config);
            }
        }


        
        //Lasting state doing state based actions like, moving
        if self.characters[character_index].character_state == CharacterState::ForwardRun {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = -(CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction());
        }
        else if self.characters[character_index].character_state == CharacterState::BackwardRun {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction();
        }
        else if self.characters[character_index].character_state == CharacterState::ForwardDash {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = -(CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction() * 2.0);
        }
        else if self.characters[character_index].character_state == CharacterState::BackwardDash {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction() * 2.0;
        }
        else if self.characters[character_index].character_state == CharacterState::LightHitRecovery {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED * self.characters[character_index].screen_side.direction();
        }
        else if self.characters[character_index].character_state == CharacterState::Parried {
            self.characters[character_index].character_velocity.y = 0.0;
            self.characters[character_index].character_velocity.x = CHARACTER_X_SPEED / 2.0 * self.characters[character_index].screen_side.direction();
        }
        else if self.characters[character_index].character_state == CharacterState::Jump {            
            self.characters[character_index].character_velocity.y = game_config.character_sheet.animations[&String::from("Jump")].displacements[self.characters[character_index].current_animation.current_frame as usize].y;
        }
        else {
            self.characters[character_index].character_velocity.x = 0.0;
            self.characters[character_index].character_velocity.y = 0.0;
        }

    }

    pub fn do_damage_to_character(&mut self, character_index: usize, amount: u32, was_a_parry: bool, game_config: &mut GameConfig) {
        match self.characters[character_index].character_state {
            CharacterState::Blocking => {
                if self.characters[character_index].health <= (amount/10) {
                    self.characters[character_index].health = 0;
                }
                else {
                    self.characters[character_index].health -= amount / 10;
                }
                self.characters[character_index].set_character_state(CharacterState::Blocking, &game_config);
            }
            _ => {
                if self.characters[character_index].health <= amount {
                    self.characters[character_index].health = 0;
                }
                else {
                    self.characters[character_index].health -= amount;
                }
                if was_a_parry  {
                    self.characters[character_index].set_character_state(CharacterState::Parried, &game_config);
                }
                else {
                    self.characters[character_index].set_character_state(CharacterState::LightHitRecovery, &game_config);
                }
            }
        }
    }

    pub fn new_with_animation_lib(animation_lib: &mut HashMap<AnimationState, AnimationConfig>) -> Round {
        //Build up the character by loading animations for each of the animation states
        let mut character_1 = Character::default();
        character_1.screen_side = ScreenSide::Right;
        character_1.character_position.x = 896.0 / 2.0 - (FRAME_WIDTH as f32 / 2.0) + (FRAME_WIDTH as f32 / 4.0); 

        character_1.current_animation = animation_lib[&AnimationState::Idle].clone();
        let mut character_2 = Character::default();
        character_2.screen_side = ScreenSide::Left;
        character_2.character_position.x = 896.0 / 2.0 - (FRAME_WIDTH as f32 / 2.0) - (FRAME_WIDTH as f32 / 4.0); // - (FRAME_WIDTH as f32) / 4.0;
        character_2.current_animation = animation_lib[&AnimationState::Idle].clone();
        Round {
            characters: vec![character_1, character_2],
            frame: 0,
            round_timer: SpriteTimer::new(60 * 60),
            round_done: false,
            hit_stun_counter: 0,
            projectiles: vec![],
            reset_round_timer: SpriteTimer::new(5 * 60)
        }
    }
}



impl Default for Round{
    fn default() -> Round {
        //Build up the character by loading animations for each of the animation states
        let mut character_1 = Character::default();
        character_1.screen_side = ScreenSide::Right;
        character_1.character_position.x = 896.0 / 2.0 - (FRAME_WIDTH as f32 / 2.0) + (FRAME_WIDTH as f32 / 4.0); 

        character_1.current_animation = AnimationConfig::new(vec![3;10]);
        let mut character_2 = Character::default();
        character_2.screen_side = ScreenSide::Left;
        character_2.character_position.x = 896.0 / 2.0 - (FRAME_WIDTH as f32 / 2.0) - (FRAME_WIDTH as f32 / 4.0); // - (FRAME_WIDTH as f32) / 4.0;
        character_2.current_animation = AnimationConfig::new(vec![3;10]);
        Round {
            characters: vec![character_1, character_2],
            frame: 0,
            round_timer: SpriteTimer::new(60 * 60),
            round_done: false,
            hit_stun_counter: 0,
            projectiles: vec![],
            reset_round_timer: SpriteTimer::new(3 * 60)
        }
    }
}