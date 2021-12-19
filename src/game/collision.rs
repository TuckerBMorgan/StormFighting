use asefile::AsepriteFile;
use storm::math::AABB2D;

use hashbrown::HashMap;

use super::{FRAME_WIDTH, AnimationState};
pub static IDLE_COLLISION: &[u8] = include_bytes!("../resources/idle.ase");
pub static FORWARD_RUN_COLLISION: &[u8] = include_bytes!("../resources/forward_run.ase");
pub static BACKWARD_RUN_COLLISION: &[u8] = include_bytes!("../resources/backward_run.ase");
pub static LIGHT_ATTACK_COLLISION: &[u8] = include_bytes!("../resources/light_attack.ase");
pub static CROUCHED_COLLISION: &[u8] = include_bytes!("../resources/crouched.ase");
pub static CROUCHING_COLLISION: &[u8] = include_bytes!("../resources/crouching.ase");

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
    Hit
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
            //See if it is a hit or hurt box
            let box_type_string = slice.user_data.unwrap().text.unwrap();
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
    /*
    //only uncomment if we have are in an enviroment where we can use files and it is better
    pub fn from_file(file_name: String) -> CollisionInfo {

        println!("Loading collision info from {:?}", file_name);
        //TODO: undo all of this when https://github.com/aseprite/aseprite/issues/721 is finished
        // Read file into memory
        let file = Path::new(file_name.as_str());

        let ase = AsepriteFile::read_file(&file).unwrap();

        //Init the memory we are gonna use
        let mut collision_info = CollisionInfo::new();

        //Turn the slice array into a slice vec, just easier to process
        let slices = ase.slices().to_vec();    
        for slice in slices {
            //See if it is a hit or hurt box
            let box_type_string = slice.user_data.unwrap().text.unwrap();
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
    */
}

pub struct CollisionLibrary {
    pub collision_info: HashMap<AnimationState, CollisionInfo>
}

impl CollisionLibrary {
    fn new() -> CollisionLibrary {
        CollisionLibrary {
            collision_info: HashMap::new()
        }
    }
    pub fn load_collision_data() -> CollisionLibrary {

        let idle = CollisionInfo::from_byte(IDLE_COLLISION);
        let forward_run = CollisionInfo::from_byte(FORWARD_RUN_COLLISION);
        let backward_run = CollisionInfo::from_byte(BACKWARD_RUN_COLLISION);
        let light_attack = CollisionInfo::from_byte(LIGHT_ATTACK_COLLISION);
        let crouched = CollisionInfo::from_byte(CROUCHED_COLLISION);
        let crouching = CollisionInfo::from_byte(CROUCHING_COLLISION);
        
        let mut collision_lib = CollisionLibrary::new();
        collision_lib.collision_info.insert(AnimationState::Idle, idle);
        collision_lib.collision_info.insert(AnimationState::ForwardRun, forward_run);
        collision_lib.collision_info.insert(AnimationState::BackwardRun, backward_run);
        collision_lib.collision_info.insert(AnimationState::LightAttack, light_attack);
        collision_lib.collision_info.insert(AnimationState::Crouched, crouched);
        collision_lib.collision_info.insert(AnimationState::Crouching, crouching);
        return collision_lib;
    }
}