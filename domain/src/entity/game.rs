pub mod id;
pub mod phase;
pub mod players;
use id::{GameId,GameIdError};
use players::Players;
use phase::GamePhase;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum GameError{
    // 不正なフェーズに変更しようとした場合
    #[error("Invalid phase: {0}")]
    InvalidPhase(String),
}

pub struct Game{
    id:GameId,
    players:Players,
    phase:GamePhase,
}

impl Game{
    pub fn change_phase(&mut self,phase:GamePhase)->Result<(),GameError>{
        match phase {
            GamePhase::RoleDetermination => {
                if self.phase != GamePhase::Result{
                    return Err(GameError::InvalidPhase(format!("{:?}",self.phase)));
                }
            },
            GamePhase::RaisingQuestion => {
                if self.phase != GamePhase::RoleDetermination{
                    return Err(GameError::InvalidPhase(format!("{:?}",self.phase)));
                }
            },
            GamePhase::Questioning => {
                if self.phase != GamePhase::RaisingQuestion{
                    return Err(GameError::InvalidPhase(format!("{:?}",self.phase)));
                }
            },
            GamePhase::Result => {
                if self.phase != GamePhase::Questioning{
                    return Err(GameError::InvalidPhase(format!("{:?}",self.phase)));
                }
            },
        }
        self.phase = phase;
        Ok(())
    }
}
