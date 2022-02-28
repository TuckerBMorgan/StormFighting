use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use storm::math::AABB2D;
use crate::*;

//A displacement is the amount alone the x and y axis a animation wants to move the character
//it may not, if for say moving along the x axis the character bumps into the edge of the map
//or the other player
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct  Displacement {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum BoxType {
    Hit,
    Hurt,
    Parry
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AABB {
    box_type: BoxType,
    origin: (i32, i32),
    size: (u32, u32)
}

impl AABB {
    pub fn new(box_type: BoxType, origin: (i32, i32), size: (u32, u32)) -> AABB {
        AABB {
            box_type,
            origin,
            size
        }
    }

    pub fn into_collision_box(&self) -> CollisionBox {
        
        let collision_box = match self.box_type {
            BoxType::Hit => {
                CollisionBoxType::Hit
            },
            BoxType::Hurt => {
                CollisionBoxType::Hurt
            },
            BoxType::Parry => {
                CollisionBoxType::Parry
            }
        };
        

        CollisionBox::new(collision_box, AABB2D::new(self.origin.0 as f32, 
                                                             self.origin.1 as f32, 
                                                             self.size.0 as f32 + self.origin.0 as f32, 
                                                             self.size.1 as f32 + self.origin.1 as f32))
    }
}

//Animation data is the information for a single Animation like "Idle" or "HeavyAttack"
//it includes the location of the sprite sheet, the Hit and HurtBoxs
//the length of time in in game frames each frame of animation will take
//and how much the character will try to move in the x and y direction those frames
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct AnimationData {
    pub image_file_location: String,
    pub collision_data: Vec<Vec<AABB>>,
    pub frame_lengths: Vec<u8>,
    pub displacements: Vec<Displacement>
}

//All the information needed to load a character into game
//there name
//how fast they are moving
//and the data for each of their animationscar
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterSheet {
    pub name: String,
    pub movespeed: f32,
    pub animations: HashMap<String, AnimationData>
}