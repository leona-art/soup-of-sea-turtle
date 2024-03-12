mod error;
mod state;
use self::error::GameError;
use self::state::GameState;
use super::{party::Party, player::Player};
use crate::common::id::Id;
use std::error::Error;

pub struct Game {
    id: Id,
    state: GameState,
    party: Party,
}

impl Game {
    pub fn new(party: Party) -> Self {
        Game {
            id: Id::new(),
            state: GameState::new(),
            party,
        }
    }
    pub fn start(&mut self) {
        self.state = GameState::Play;
    }
    pub fn end(&mut self) {
        self.state = GameState::End;
    }
}
