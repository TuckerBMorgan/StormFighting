use asefile::AsepriteFile;
use storm::math::AABB2D;

use hashbrown::HashMap;

use super::{FRAME_WIDTH, AnimationState};
pub static IDLE_COLLISION: &[u8] = include_bytes!("../resources/idle.ase");
pub static FORWARD_RUN_COLLISION: &[u8] = include_bytes!("../resources/forward_run.ase");
pub static BACKWARD_RUN_COLLISION: &[u8] = include_bytes!("../resources/backward_run.ase");
pub static LIGHT_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/light_attack.ase");
pub static MEDIUM_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/medium_attack.ase");
pub static HEAVY_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/heavy_attack.ase");
pub static CROUCHED_COLLISION: &[u8] = include_bytes!("../resources/crouched.ase");
pub static CROUCHING_COLLISION: &[u8] = include_bytes!("../resources/crouching.ase");
pub static LIGHT_CROUCH_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/light_crouch_attack.ase");
pub static HEAVY_CROUCH_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/heavy_crouching_attack.ase");
pub static LIGHT_KICK_COLLISION: &[u8] = include_bytes!("../resources/light_kick.ase");
pub static MEDIUM_KICK_COLLISION: &[u8] = include_bytes!("../resources/medium_kick.ase");
pub static HEAVY_KICK_COLLISION: &[u8] = include_bytes!("../resources/heavy_kick.ase");
pub static FORWARD_DASH_COLLISION: &[u8] = include_bytes!("../resources/forward_dash.ase");
pub static BACKWARD_DASH_COLLISION: &[u8] = include_bytes!("../resources/backward_dash.ase");
pub static SPECIAL_1_COLLISION: &[u8] = include_bytes!("../resources/special_1.ase");
pub static FIREBALL_COLLISION: &[u8] = include_bytes!("../resources/fireball_main.ase");

pub trait Reflect {
    fn reflect(&self, x_axis: usize) -> AABB2D;
}

impl Reflect for AABB2D {
    fn reflect(&self, x_axis: usize) -> AABB2D {
        let min_dif = self.min.x - x_axis as f32;
        let max_dif = self.max.x - x_axis as f32;
        AABB2D::new(
            self.min.x - (min_dif + max_dif),
            self.min.y,
            self.max.x - (min_dif + max_dif),
            self.max.y)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CollisionBoxType {
    Hurt,
    Hit,
    Parry
}
#[derive(Debug)]
pub struct CollisionBox {
    pub box_type: CollisionBoxType,
    pub aabb: AABB2D
}

impl CollisionBox {
    pub fn new(box_type: CollisionBoxType, aabb: AABB2D) -> CollisionBox {
        CollisionBox {
            box_type,
            aabb
        }
    }
}

pub enum CharacterNumber {
    Number1,
    Number2
}

pub struct CollisionReport {
    pub collider_type: CollisionBoxType,
    pub collide_type: CollisionBoxType,
    pub collider_character: CharacterNumber
}

impl CollisionReport {
    pub fn new(collider_type: CollisionBoxType, collide_type: CollisionBoxType, collider_character: CharacterNumber) -> CollisionReport {
        CollisionReport {
            collider_type,
            collide_type,
            collider_character
        }
    }
}

#[derive(Debug)]
pub struct CollisionInfo {
    pub frame_collision: HashMap<u32, Vec<CollisionBox>>
}

impl CollisionInfo {
    fn new() -> CollisionInfo {
        CollisionInfo {
            frame_collision: HashMap::new()
        }
    }

    pub fn from_byte(bytes: &[u8]) -> CollisionInfo {
        //TODO: undo all of this when https://github.com/aseprite/aseprite/issues/721 is finished
        // Read file into memory
        let ase = AsepriteFile::read(bytes).unwrap();
        //let ase = AsepriteFile::read_file(&file).unwrap();

        //Init the memory we are gonna use
        let mut collision_info = CollisionInfo::new();

        //Turn the slice array into a slice vec, just easier to process
        let slices = ase.slices().to_vec();    
        for slice in slices {
            let slice_name = slice.name;
            //See if it is a hit or hurt box
            let box_type_string = slice.user_data.expect(&slice_name).text.expect(&slice_name);
            let box_type;
            if box_type_string == "HitBox" {
                box_type = CollisionBoxType::Hit;
            }
            else {
                box_type = CollisionBoxType::Hurt;
            }

            //Grab the bounds of the slice
            let a = &slice.keys[0];
            let b = a.origin;
            let c = a.size;

            //Then we calculate which FRAME the collision box is a part of
            //Since Aseprite does not let you assign slices to frame we have
            //Calucalte where it is on the whole strip and then assing it to that frame
            //building the frame index out selves

            let index = (b.0 as u32) / FRAME_WIDTH;
            let offset = index * FRAME_WIDTH;
            //Because we are doing it this way we also need to normalize the x coordinate of the AABB
            //Y is fine
            let fixed_x_min = b.0 as f32 - offset as f32;
            let aabb = AABB2D::new(fixed_x_min, 
                                          b.1 as f32 , 
                                          c.0 as f32 + fixed_x_min, 
                                          c.1 as f32 + b.1 as f32);
            if collision_info.frame_collision.contains_key(&index) == false {
                collision_info.frame_collision.insert(index, vec![]);
            }
            //Combine the two parts of animation
            collision_info.frame_collision.get_mut(&index).unwrap().push(CollisionBox::new(box_type, aabb));
        }
        return collision_info;
    }
}

pub struct CollisionLibrary {
    pub collision_info: HashMap<AnimationState, CollisionInfo>,
    pub fireball_collision: CollisionInfo
}
impl Default for CollisionLibrary {
    fn default() -> CollisionLibrary {
        let idle = CollisionInfo::from_byte(IDLE_COLLISION);
        let forward_run = CollisionInfo::from_byte(FORWARD_RUN_COLLISION);
        let backward_run = CollisionInfo::from_byte(BACKWARD_RUN_COLLISION);
        let light_attack = CollisionInfo::from_byte(LIGHT_ATTACK_COLLISION);
        let medium_attack = CollisionInfo::from_byte(MEDIUM_ATTACK_COLLISION);
        let heavy_attack = CollisionInfo::from_byte(HEAVY_ATTACK_COLLISION);
        let crouched = CollisionInfo::from_byte(CROUCHED_COLLISION);
        let crouching = CollisionInfo::from_byte(CROUCHING_COLLISION);
        let light_crouch_attack = CollisionInfo::from_byte(LIGHT_CROUCH_ATTACK_COLLISION);
        let heavy_crouch_attack = CollisionInfo::from_byte(HEAVY_CROUCH_ATTACK_COLLISION);
        let light_kick = CollisionInfo::from_byte(LIGHT_KICK_COLLISION);
        let medium_kick = CollisionInfo::from_byte(MEDIUM_KICK_COLLISION);
        let heavy_kick = CollisionInfo::from_byte(HEAVY_KICK_COLLISION);
        let forward_dash = CollisionInfo::from_byte(FORWARD_DASH_COLLISION);
        let backward_dash = CollisionInfo::from_byte(BACKWARD_DASH_COLLISION);
        let special_1 = CollisionInfo::from_byte(SPECIAL_1_COLLISION);

        let mut collision_lib = CollisionLibrary::new();
        collision_lib.collision_info.insert(AnimationState::Idle, idle);
        collision_lib.collision_info.insert(AnimationState::ForwardRun, forward_run);
        collision_lib.collision_info.insert(AnimationState::BackwardRun, backward_run);
        collision_lib.collision_info.insert(AnimationState::LightAttack, light_attack);
        collision_lib.collision_info.insert(AnimationState::MediumAttack, medium_attack);
        collision_lib.collision_info.insert(AnimationState::HeavyAttack, heavy_attack);
        collision_lib.collision_info.insert(AnimationState::Crouched, crouched);
        collision_lib.collision_info.insert(AnimationState::Crouching, crouching);
        collision_lib.collision_info.insert(AnimationState::LightCrouchAttack, light_crouch_attack);
        collision_lib.collision_info.insert(AnimationState::HeavyCrouchingAttack, heavy_crouch_attack);
        collision_lib.collision_info.insert(AnimationState::LightKick, light_kick);
        collision_lib.collision_info.insert(AnimationState::MediumKick, medium_kick);
        collision_lib.collision_info.insert(AnimationState::HeavyKick, heavy_kick);
        collision_lib.collision_info.insert(AnimationState::ForwardDash, forward_dash);
        collision_lib.collision_info.insert(AnimationState::BackwardDash, backward_dash);
        collision_lib.collision_info.insert(AnimationState::Special1, special_1);

        return collision_lib;
    }
}

impl CollisionLibrary {
    fn new() -> CollisionLibrary {
        CollisionLibrary {
            collision_info: HashMap::new(),
            fireball_collision: CollisionInfo::from_byte(FIREBALL_COLLISION)
        }
    }

}