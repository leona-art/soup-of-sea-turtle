use crate::common::{id::Id, player_name::PlayerName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub id: Id,
    pub name: PlayerName,
}

impl Player {
    pub fn new(name: PlayerName) -> Player {
        Player {
            id: Id::new(),
            name,
        }
    }
}
