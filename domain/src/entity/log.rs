use self::{id::LogId, qa::QA};

use super::game::{answer::Answer, players::Player, question::Question};

pub mod id;
pub mod qa;
pub mod repository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameLog {
    pub id: LogId,
    pub qas: Vec<QA>,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            id:LogId::new(),
            qas: Vec::new(),
        }
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn add_qa(&mut self, player:&Player,q:&Question,a:&Answer) {
        let qa=QA::new(player.clone(),q.clone(),a.clone());
        self.qas.push(qa);
    }
}
