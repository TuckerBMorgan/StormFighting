use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize,)]
pub enum StateName {
    Empty,
    Idle,
    Running
}

pub trait State {
    fn get_state_name(&self) -> StateName;
    fn should_transition(&self) -> Option<StateName>;
    fn tick(&mut self);
    fn enter_state(&mut self);
    fn exit_state(&mut self);
}

#[derive(Serialize, Deserialize)]
pub struct StateMachine {
    states: HashMap<StateName, Box<dyn State>>,
    current_state: StateName
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            states: HashMap::new(),
            current_state: StateName::Empty
        }
    }

    pub fn tick(&mut self) {
        match self.current_state {
            StateName::Idle | StateName::Running => {
                let state_name = self.current_state;
                self.states.get_mut(&state_name).unwrap().tick();
                let should_transition = self.states.get(&state_name).unwrap().should_transition();
                match should_transition {
                    Some(state_name) => {
                        self.current_state = state_name;
                    },
                    None => {

                    }
                }
            },
            StateName::Empty => {

            }
        }
    }

    pub fn add_state(&mut self, state: Box<dyn State>) {
        self.states.insert(state.get_state_name(), state);
    }
}