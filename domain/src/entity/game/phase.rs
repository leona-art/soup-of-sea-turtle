// ゲームフェース
// 1. 役割決め
// 2. 問題出題
// 3. 問答
// 4. 終了

use super::{
    answer::Answer,
    players::{Player, Players, Role},
    question::Question,
    subject::Subject,
};
use crate::entity::{log::GameLog, user::User};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GamePhase {
    SelectRole(SelectRolePhase),
    SelectSubject(SelectSubject),
    Questioning,
    Result,
}

impl GamePhase {
    pub fn is_next(&self, phase: &Self) -> bool {
        match (self, phase) {
            (GamePhase::SelectRole(_), GamePhase::SelectSubject(_)) => true,
            (GamePhase::SelectSubject(_), GamePhase::Questioning) => true,
            (GamePhase::Questioning, GamePhase::Result) => true,
            (GamePhase::Result, GamePhase::SelectRole(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SelectRolePhase(Players);
impl SelectRolePhase {
    pub fn new() -> Self {
        Self(Players::new())
    }
}
impl SelectRolePhase {
    pub fn select_role(&mut self, user: &User, role: Role) {
        let player = Player::new(user.clone(), role);
        self.0.add(player);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SelectSubject {
    players: Players,
    subject: Option<Subject>,
}
impl SelectSubject {
    pub fn select_question(&mut self, subject: Subject) {
        self.subject = Some(subject);
    }
}
impl From<SelectRolePhase> for SelectSubject {
    fn from(phase: SelectRolePhase) -> Self {
        Self {
            players: phase.0,
            subject: None,
        }
    }
}
impl From<Result> for SelectSubject {
    fn from(phase: Result) -> Self {
        Self {
            players: phase.players,
            subject: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Questioning {
    players: Players,
    subject: Subject,
    log: GameLog,
}

impl Questioning {
    pub fn logging(&mut self, player: &Player, question: &Question, answer: &Answer) {
        self.log.add_qa(player, question, answer);
    }
}
impl From<SelectSubject> for Questioning {
    fn from(phase: SelectSubject) -> Self {
        Self {
            players: phase.players,
            subject: phase.subject.unwrap(),
            log: GameLog::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Result {
    players: Players,
    log: GameLog,
}

impl From<Questioning> for Result {
    fn from(phase: Questioning) -> Self {
        Self {
            players: phase.players,
            log: phase.log,
        }
    }
}
