
use storm::event::*;
use super::*;
use serde::{Deserialize, Serialize};
use bytemuck::{Pod, Zeroable};

//To reduce the size of the input we send between 
//players we fit it onto a u16
//We use this to set the position on that u16
//that each input is
pub const INPUT_LIGHT_ATTACK: u16 = 1 << 0;
pub const INPUT_LEFT: u16 = 1 << 1;
pub const INPUT_RIGHT: u16 = 1 << 2;
pub const INPUT_DOWN: u16 = 1 << 3;
pub const INPUT_MEDIUM_ATTACK: u16 = 1 << 4;
pub const INPUT_HEAVY_ATTACK: u16 = 1 << 5;
pub const INPUT_LIGHT_KICK: u16 = 1 << 6;
pub const INPUT_MEDIUM_KICK: u16 = 1 << 7;
pub const INPUT_HEAVY_KICK: u16 = 1 << 9;
pub const INPUT_JUMP: u16 = 1 << 8; 

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct NetInput {
    pub input: u16
}


#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Default, Copy, Clone, Debug)]
pub struct Input {
    pub left_key_down:  bool,
    pub right_key_down: bool,
    pub down_key_down:  bool,
    pub light_attack:   bool,
    pub medium_attack:  bool,
    pub heavy_attack:   bool,
    pub light_kick:     bool,
    pub medium_kick:    bool,
    pub heavy_kick:     bool,
    pub jump_down:      bool,
    pub has_input:      bool
}

impl Input {
    pub fn new() -> Input {
        Input {
            left_key_down:  false,
            right_key_down: false,
            down_key_down:  false,
            light_attack:   false,
            medium_attack:  false,
            heavy_attack:   false,
            light_kick:     false,
            medium_kick:    false,
            heavy_kick:     false,
            jump_down:      false,
            has_input:      false
        }
    }

    pub fn from_game_input(game_input: NetInput) -> Input {
        let recombined_input = game_input.input;
        let has_input = recombined_input > 0;
        Input {
            left_key_down:  (recombined_input & INPUT_LEFT) != 0,
            right_key_down: (recombined_input & INPUT_RIGHT) != 0,
            down_key_down:  (recombined_input & INPUT_DOWN) != 0,
            light_attack:   (recombined_input & INPUT_LIGHT_ATTACK) != 0,
            medium_attack:  (recombined_input & INPUT_MEDIUM_ATTACK) != 0,
            heavy_attack:   (recombined_input & INPUT_HEAVY_ATTACK) != 0,
            light_kick:     (recombined_input & INPUT_LIGHT_KICK) != 0,
            medium_kick:    (recombined_input & INPUT_MEDIUM_KICK) != 0,
            heavy_kick:     (recombined_input & INPUT_HEAVY_KICK) != 0,
            jump_down:      (recombined_input & INPUT_JUMP) != 0,
            has_input
        }
    }

    //TODO: let this be configurable so we can handle
    //Controllers, key rebinds, etc
    pub fn key_down(&mut self, keyboard_button: KeyboardButton) {
        match keyboard_button {
            KeyboardButton::Left => {
                self.left_key_down = true;
            },
            KeyboardButton::Right => {
                self.right_key_down = true;
            },
            KeyboardButton::Q => {
                self.light_attack = true;
            },
            KeyboardButton::Down => {
                self.down_key_down = true;
            },
            KeyboardButton::W => {
                self.medium_attack = true;
            },
            KeyboardButton::E => {
                self.heavy_attack = true;
            }
            KeyboardButton::A => {
                self.light_kick = true;
            }
            KeyboardButton::S => {
                self.medium_kick = true;
            }
            KeyboardButton::D => {
                self.heavy_kick = true;
            }
            KeyboardButton::Space => {
                self.jump_down = true;
            }
            _ => {}
        }
    }

    pub fn key_up(&mut self, keyboard_button: KeyboardButton) {
        match keyboard_button {
            KeyboardButton::Left => {
                self.left_key_down = false;
            },
            KeyboardButton::Right => {
                self.right_key_down = false;
            },
            KeyboardButton::Q => {
                self.light_attack = false;
            },
            KeyboardButton::Down => {
                self.down_key_down = false;
            }
            KeyboardButton::W => {
                self.medium_attack = false;
            },
            KeyboardButton::E => {
                self.heavy_attack = false;
            }
            KeyboardButton::A => {
                self.light_kick = false;
            }
            KeyboardButton::S => {
                self.medium_kick = false;
            }
            KeyboardButton::D => {
                self.heavy_kick = false;
            }
            KeyboardButton::Space => {
                self.jump_down = false;
            }
            _ => {}
        }
    }
}


//Similar to the input struct, but will adjust the left and right keys down
//To Forward and Backward, to allow for side adnostic code
#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Default, Copy, Clone, Debug)]
pub struct ScreenSideAdjustedInput {
    pub forward_down:   bool,
    pub backward_down:  bool,
    pub down_key_down:  bool,
    pub light_attack:   bool,
    pub medium_attack:  bool,
    pub heavy_attack:   bool,
    pub light_kick:     bool,
    pub medium_kick:    bool,
    pub heavy_kick:     bool,
    pub has_input:      bool,
    pub jump:           bool
}

impl ScreenSideAdjustedInput {
    pub fn new(input: &Input, screen_side: ScreenSide) -> ScreenSideAdjustedInput {
        let mut forward = false;
        if screen_side == ScreenSide::Left && input.right_key_down {
            forward = true;
        }
        if screen_side == ScreenSide::Right && input.left_key_down {
            forward = true;
        }
        let mut backward = false;
        if screen_side == ScreenSide::Left && input.left_key_down {
            backward = true;
        }
        if screen_side == ScreenSide::Right && input.right_key_down {
            backward = true;
        }

        ScreenSideAdjustedInput {
            forward_down: forward,
            backward_down: backward,
            down_key_down:  input.down_key_down,
            light_attack:   input.light_attack,
            medium_attack:  input.medium_attack,
            heavy_attack:   input.heavy_attack,
            light_kick:     input.light_kick,
            medium_kick:    input.medium_kick,
            heavy_kick:     input.heavy_kick,
            has_input:      input.has_input,
            jump:           input.jump_down
        }
    }
}

pub struct PatternElement {
    character_action: CharacterAction,
    start: bool,
    confirm: bool,
    final_element: bool
}

impl PatternElement {
    
    pub fn new(character_action: CharacterAction) -> PatternElement {
        PatternElement {
            character_action,
            start: false,
            confirm: false,
            final_element: false
        }
    }

    pub fn process_input(&mut self, input: &ScreenSideAdjustedInput) -> bool {
        let button_down_in_this_frame;
        match self.character_action {
            CharacterAction::Crouch => {
                button_down_in_this_frame = input.down_key_down;
            },
            CharacterAction::MoveForward => {
                button_down_in_this_frame = input.forward_down;
            },
            CharacterAction::MoveBackward => {
                button_down_in_this_frame = input.backward_down;
            },
            CharacterAction::LightAttack => {
                button_down_in_this_frame = input.light_attack;
            },
            CharacterAction::MediumAttack => {
                button_down_in_this_frame = input.medium_attack;
            },
            CharacterAction::HeavyAttack => {
                button_down_in_this_frame = input.heavy_attack;
            },
            CharacterAction::LightKick => {
                button_down_in_this_frame = input.light_kick;
            },
            CharacterAction::MediumKick => {
                button_down_in_this_frame = input.medium_kick;
            },
            CharacterAction::HeavyKick => {
                button_down_in_this_frame = input.heavy_kick;
            },
            CharacterAction::Jump => {
                button_down_in_this_frame = input.jump;
            }
            _ => {
                button_down_in_this_frame = false;
            }
        }

        if self.start == false && button_down_in_this_frame == false {
            self.start = true;
            return false;
        }
        else if self.start == true && button_down_in_this_frame == true {
            self.confirm = true;
            if self.final_element == true {
                return true;
            }
            return false;
        }
        if self.start && self.confirm && (self.final_element || button_down_in_this_frame == false) {
            return true;
        }
        return false;

    }

    pub fn reset(&mut self) {
        self.start = false;
        self.confirm = false;
    }
}

pub struct ComboPattern {
    pattern: Vec<PatternElement>,
    matched_pattern_number: usize,
    result_action: CharacterAction
}

impl ComboPattern {

    pub fn new(pattern: Vec<CharacterAction>, result_action: CharacterAction) -> ComboPattern {
        let mut pattern: Vec<PatternElement> = pattern.iter().map(|x|PatternElement::new(*x)).collect();
        let index = pattern.len() - 1;
        pattern[index].final_element = true;
        ComboPattern {
            pattern,
            matched_pattern_number: 0,
            result_action
        }
    }

    pub fn process_input(&mut self, input: &ScreenSideAdjustedInput) -> Option<CharacterAction> {
        if self.matched_pattern_number != self.pattern.len() 
            && self.pattern[self.matched_pattern_number].process_input(input) {
            self.matched_pattern_number += 1;
            if self.matched_pattern_number == self.pattern.len() {
                return Some(self.result_action);
            }
        }
        return None;
    }

    pub fn reset(&mut self) {
        self.matched_pattern_number = 0;
        for pattern in &mut self.pattern {
            pattern.reset();
        }
    }   
}

pub struct ComboLibrary {
    pub combos: Vec<ComboPattern>
}

impl ComboLibrary {
    pub fn reset(&mut self) {
        for combo in self.combos.iter_mut() {
            combo.reset();
        }
    }
}

impl Default for ComboLibrary {
    fn default() -> ComboLibrary {
        
        let forward_dash = ComboPattern::new(vec![CharacterAction::MoveForward, CharacterAction::MoveForward], CharacterAction::DashForward);
        let backward_dash = ComboPattern::new(vec![CharacterAction::MoveBackward, CharacterAction::MoveBackward], CharacterAction::DashBackward);
        let hadokon = ComboPattern::new(vec![CharacterAction::Crouch, CharacterAction::MoveForward, CharacterAction::LightAttack], CharacterAction::Special1);
        let forwrad_jump = ComboPattern::new(vec![CharacterAction::MoveForward, CharacterAction::Jump], CharacterAction::ForwardJump);
        //let backward_jump = ComboPattern::new(vec![CharacterAction::MoveForward, CharacterAction::Jump], CharacterAction::ForwardJump);

        ComboLibrary {
            combos: vec![forward_dash, backward_dash, hadokon, forwrad_jump]
        }
    }
}