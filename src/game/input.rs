use ggrs::GameInput;
use storm::*;
use serde::{Deserialize, Serialize};

pub const INPUT_LIGHT_ATTACK: u16 = 1 << 0;
pub const INPUT_LEFT: u16 = 1 << 1;
pub const INPUT_RIGHT: u16 = 1 << 2;
pub const INPUT_DOWN: u16 = 1 << 3;
pub const INPUT_MEDIUM_ATTACK: u16 = 1 << 4;
pub const INPUT_HEAVY_ATTACK: u16 = 1 << 5;
pub const INPUT_LIGHT_KICK: u16 = 1 << 6;
pub const INPUT_MEDIUM_KICK: u16 = 1 << 7;
pub const INPUT_HEAVY_KICK: u16 = 1 << 8;

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Input {
    pub left_key_down:  bool,
    pub right_key_down: bool,
    pub down_key_down:  bool,
    pub light_attack:   bool,
    pub medium_attack:  bool,
    pub heavy_attack:   bool,
    pub light_kick:     bool,
    pub medium_kick:    bool,
    pub heavy_kick:     bool
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
            heavy_kick:     false
        }
    }

    pub fn from_game_input(game_input: GameInput) -> Input {
        let mut recombined_input = game_input.buffer[0] as u16;
        let mut second_half = game_input.buffer[1] as u16;
        second_half = second_half << 8;
        recombined_input = recombined_input | second_half;
        Input {
            left_key_down:  (recombined_input & INPUT_LEFT) != 0,
            right_key_down: (recombined_input & INPUT_RIGHT) != 0,
            down_key_down:  (recombined_input & INPUT_DOWN) != 0,
            light_attack:   (recombined_input & INPUT_LIGHT_ATTACK) != 0,
            medium_attack:  (recombined_input & INPUT_MEDIUM_ATTACK) != 0,
            heavy_attack:   (recombined_input & INPUT_HEAVY_ATTACK) != 0,
            light_kick:     (recombined_input & INPUT_LIGHT_KICK) != 0,
            medium_kick:    (recombined_input & INPUT_MEDIUM_KICK) != 0,
            heavy_kick:     (recombined_input & INPUT_HEAVY_KICK) != 0
        }
    }

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
