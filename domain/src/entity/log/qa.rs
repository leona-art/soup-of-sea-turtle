use crate::entity::game::{answer::Answer, players::Player, question::Question};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QA {
    pub player: Player,
    pub question: Question,
    pub answer: Answer,
}

impl QA {
    pub fn new(player: Player, question: Question, answer: Answer) -> Self {
        Self {
            player,
            question,
            answer,
        }
    }
}
