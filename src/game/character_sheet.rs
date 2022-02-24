use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

//A displacement is the amount alone the x and y axis a animation wants to move the character
//it may not, if for say moving along the x axis the character bumps into the edge of the map
//or the other player
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct  Displacement {
    pub x: f32,
    pub y: f32
}

//Animation data is the information for a single Animation like "Idle" or "HeavyAttack"
//it includes the location of the sprite sheet, the Hit and HurtBoxs
//the length of time in in game frames each frame of animation will take
//and how much the character will try to move in the x and y direction those frames
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct AnimationData {
    pub image_file_location: String,
    pub collision_file_location: String,
    pub frame_lengths: Vec<u8>,
    pub displacements: Vec<Displacement>
}

//All the information needed to load a character into game
//there name
//how fast they are moving
//and the data for each of their animations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterSheet {
    pub name: String,
    pub movespeed: f32,
    pub animations: HashMap<String, AnimationData>
}
