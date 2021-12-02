use storm::*;

use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell, PlayerHandle, NULL_FRAME};

use serde::{Deserialize, Serialize};
use super::{Round, Input, CollisionLibrary};

pub const CHECKSUM_PERIOD: i32 = 100;

pub const INPUT_LIGHT_ATTACK: u8 = 1 << 0;
pub const INPUT_LEFT: u8 = 1 << 1;
pub const INPUT_RIGHT: u8 = 1 << 2;

/// computes the fletcher16 checksum, copied from wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for index in 0..data.len() {
        sum1 = (sum1 + data[index] as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}


pub struct Game {
    pub current_round: Round,
    pub local_input: Input,
    pub last_checksum: (Frame, u64),
    pub periodic_checksum: (Frame, u64),
    pub collision_library: CollisionLibrary
}

impl Game {
    pub fn new(current_round: Round) -> Game {
        Game {
            current_round,
            local_input: Input::new(),
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            collision_library: CollisionLibrary::load_collision_data()
        }
    }
    
    pub fn key_down(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_down(keyboard_button);
    }

    pub fn key_up(&mut self, keyboard_button: KeyboardButton) {
        self.local_input.key_up(keyboard_button);
    }

    // deserialize gamestate to load and overwrite current gamestate
    pub fn load_game_state(&mut self, cell: GameStateCell) {
        let state_to_load = cell.load();
        self.current_round = bincode::deserialize(&state_to_load.buffer.unwrap()).unwrap();
    }

    // serialize current gamestate, create a checksum
    // creating a checksum here is only relevant for SyncTestSessions
    fn save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        // assert_eq!(self.game_state.frame, frame);
        let buffer = bincode::serialize(&self.current_round).unwrap();
        let checksum = fletcher16(&buffer) as u64;

        cell.save(GameState::new(frame, Some(buffer), Some(checksum)));
    }

    
    fn advance_frame(&mut self, inputs: Vec<GameInput>) {
        // advance the game state
        self.current_round.advance(inputs, &self.collision_library);

        // remember checksum to render it later
        // it is very inefficient to serialize the gamestate here just for the checksum
        let buffer = bincode::serialize(&self.current_round).unwrap();
        let checksum = fletcher16(&buffer) as u64;
        self.last_checksum = (self.current_round.frame, checksum);
        if self.current_round.frame % CHECKSUM_PERIOD == 0 {
            self.periodic_checksum = (self.current_round.frame, checksum);
        }
    }

    // for each request, call the appropriate function
    pub fn handle_requests(&mut self, requests: Vec<GGRSRequest>) {
        for request in requests {
            match request {
                GGRSRequest::LoadGameState { cell } => self.load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => self.save_game_state(cell, frame),
                GGRSRequest::AdvanceFrame { inputs } => self.advance_frame(inputs),
            }
        }
    }

    #[allow(dead_code)]
    // creates a compact representation of currently pressed keys and serializes it
    pub fn local_input(&self, handle: PlayerHandle) -> Vec<u8> {
        let mut input: u8 = 0;

        // ugly, but it works...
        // player 1 with WASD
        if handle == 0 {
            if self.local_input.left_key_down {
                input |= INPUT_LEFT;
            }
            if self.local_input.right_key_down {
                input |= INPUT_RIGHT;
            }
            if self.local_input.light_attack {
                input |= INPUT_LIGHT_ATTACK;
            }
        }
        vec![input]
    }
}

impl Default for Game {
    fn default() -> Game {
        Game {
            current_round: Round::default(),
            local_input: Input::new(),
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            collision_library: CollisionLibrary::load_collision_data()
        }
    }
}