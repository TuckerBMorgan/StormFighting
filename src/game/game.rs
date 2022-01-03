
use hashbrown::HashMap;
use storm::event::*;
use ggrs::{Frame, GGRSRequest, GameInput, GameState, GameStateCell, PlayerHandle, NULL_FRAME};

use super::*;
use super::character::AnimationStateForCharacterState;

pub const CHECKSUM_PERIOD: i32 = 100;


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

pub struct GameConfig {
    pub collision_library: CollisionLibrary,
    pub combo_library: ComboLibrary,
    pub animation_for_character_state_library: HashMap<CharacterState, AnimationStateForCharacterState>
}

impl GameConfig {
    pub fn new(collision_library: CollisionLibrary, combo_library: ComboLibrary, animation_for_character_state_library: HashMap<CharacterState, AnimationStateForCharacterState>) -> GameConfig {
        GameConfig {
            collision_library,
            combo_library,
            animation_for_character_state_library
        }
    }
}

pub struct Game {
    pub current_round: Round,
    pub local_input: Input,
    pub last_checksum: (Frame, u64),
    pub periodic_checksum: (Frame, u64),
    pub game_config: GameConfig
}

impl Game {
    
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
        self.current_round.advance(inputs, &mut self.game_config);
        if self.current_round.round_done {
            self.current_round = Round::default();
        }

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
        let mut input: u16 = 0;

        // ugly, but it works...
        // player 1 with WASD
        if handle == 0 {
            if self.local_input.left_key_down {
                input |= INPUT_LEFT;
            }
            if self.local_input.right_key_down {
                input |= INPUT_RIGHT;
            }
            if self.local_input.down_key_down {
                input |= INPUT_DOWN;
            }
            if self.local_input.light_attack {
                input |= INPUT_LIGHT_ATTACK;
            }
            if self.local_input.medium_attack {
                input |= INPUT_MEDIUM_ATTACK;
            }
            if self.local_input.heavy_attack {
                input |= INPUT_HEAVY_ATTACK;
            }
            if self.local_input.light_kick {
                input |= INPUT_LIGHT_KICK;
            }
            if self.local_input.medium_kick {
                input |= INPUT_MEDIUM_KICK;
            }
            if self.local_input.heavy_kick {
                input |= INPUT_HEAVY_KICK;
            }
        }
        return input.to_le_bytes().to_vec();
    }
}

impl Default for Game {
    fn default() -> Game {

        let mut animation_for_character_state_library = HashMap::new();
        animation_for_character_state_library.insert(CharacterState::Idle, AnimationStateForCharacterState::new(AnimationState::Crouched, AnimationState::Idle));
        animation_for_character_state_library.insert(CharacterState::ForwardRun, AnimationStateForCharacterState::new(AnimationState::ForwardRun, AnimationState::ForwardRun));
        animation_for_character_state_library.insert(CharacterState::BackwardRun, AnimationStateForCharacterState::new(AnimationState::BackwardRun, AnimationState::BackwardRun));
        animation_for_character_state_library.insert(CharacterState::LightHitRecovery, AnimationStateForCharacterState::new(AnimationState::LightHitRecovery, AnimationState::LightHitRecovery));
        animation_for_character_state_library.insert(CharacterState::Blocking, AnimationStateForCharacterState::new(AnimationState::Blocking, AnimationState::Blocking));
        animation_for_character_state_library.insert(CharacterState::Crouching, AnimationStateForCharacterState::new(AnimationState::Crouching, AnimationState::Crouching));
        animation_for_character_state_library.insert(CharacterState::LightAttack, AnimationStateForCharacterState::new(AnimationState::LightCrouchAttack, AnimationState::LightAttack));
        animation_for_character_state_library.insert(CharacterState::MediumAttack, AnimationStateForCharacterState::new(AnimationState::LightCrouchAttack, AnimationState::MediumAttack));
        animation_for_character_state_library.insert(CharacterState::HeavyAttack, AnimationStateForCharacterState::new(AnimationState::HeavyCrouchingAttack, AnimationState::HeavyAttack));
        animation_for_character_state_library.insert(CharacterState::LightKick, AnimationStateForCharacterState::new(AnimationState::LightKick, AnimationState::LightKick));
        animation_for_character_state_library.insert(CharacterState::MediumKick, AnimationStateForCharacterState::new(AnimationState::MediumKick, AnimationState::MediumKick));
        animation_for_character_state_library.insert(CharacterState::HeavyKick, AnimationStateForCharacterState::new(AnimationState::HeavyKick, AnimationState::HeavyKick));
        animation_for_character_state_library.insert(CharacterState::ForwardDash, AnimationStateForCharacterState::new(AnimationState::ForwardDash, AnimationState::ForwardDash));
        animation_for_character_state_library.insert(CharacterState::BackwardDash, AnimationStateForCharacterState::new(AnimationState::BackwardDash, AnimationState::BackwardDash));
        animation_for_character_state_library.insert(CharacterState::Special1, AnimationStateForCharacterState::new(AnimationState::Special1, AnimationState::Special1));
        let game_config = GameConfig::new(CollisionLibrary::default(), ComboLibrary::default(), animation_for_character_state_library);
        Game {
            current_round: Round::default(),
            local_input: Input::new(),
            last_checksum: (NULL_FRAME, 0),
            periodic_checksum: (NULL_FRAME, 0),
            game_config
        }
    }
}