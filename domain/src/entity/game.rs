pub mod answer;
pub mod id;
pub mod phase;
pub mod players;
pub mod question;
pub mod repository;
pub mod subject;
use id::GameId;
use phase::{GamePhase, SelectRolePhase};
use thiserror::Error;

use super::user::User;

#[derive(Error, Debug)]
pub enum GameError {
    // 不正なフェーズに変更しようとした場合
    #[error("Invalid phase: {0}")]
    InvalidPhase(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    id: GameId,
    users: Vec<User>,
    phase: GamePhase,
}

impl Game {
    pub fn new(users: &[User]) -> Self {
        Self {
            id: GameId::new(),
            users: users.to_vec(),
            phase: GamePhase::SelectRole(SelectRolePhase::new()),
        }
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn change_phase(&mut self, phase: GamePhase) -> Result<(), GameError> {
        if self.phase.is_next(&phase) {
            return Err(GameError::InvalidPhase(format!(
                "{:?} -> {:?}",
                self.phase, phase
            )));
        }

        self.phase = phase;
        Ok(())
    }
}
