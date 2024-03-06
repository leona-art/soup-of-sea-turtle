use super::game_room::GameRoom;
use crate::common::id::Id;
pub struct Host {
    id: Id,
    pub game_room: Option<GameRoom>,
}

impl Host {
    pub fn new() -> Host {
        Host {
            id: Id::new(),
            game_room: None,
        }
    }
    pub fn create_game_room(&mut self, name: String) {
        self.game_room = Some(GameRoom::new(name));
    }

    pub fn remove_game_room(&mut self) {
        self.game_room = None;
    }
}