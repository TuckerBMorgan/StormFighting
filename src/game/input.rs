use ggrs::GameInput;
use storm::event::*;
use super::*;
use serde::{Deserialize, Serialize};

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
pub const INPUT_HEAVY_KICK: u16 = 1 << 8;


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
            has_input:      false
        }
    }

    pub fn from_game_input(game_input: GameInput) -> Input {
        let mut recombined_input = game_input.buffer[0] as u16;
        let mut second_half = game_input.buffer[1] as u16;
        second_half = second_half << 8;
        recombined_input = recombined_input | second_half;
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
    pub has_input:      bool
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
            has_input:      input.has_input
        }
    }
}