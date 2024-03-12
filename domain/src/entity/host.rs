use super::player::{IsHost, Player};
use crate::common::id::Id;
pub mod room_manager;
use super::host::room_manager::RoomManager;

pub struct Host {
    pub id: Id,
    pub name: String,
    pub room_manager: RoomManager,
}

impl Host {
    pub fn new(name: &str) -> Self {
        Host {
            id: Id::new(),
            name: name.to_string(),
            room_manager: RoomManager::new(),
        }
    }
    pub fn create_game_room(&mut self, name: String) {
        self.room_manager.create_room(name, self);
    }

    pub fn remove_game_room(&mut self) {
        self.room_manager.remove_room();
    }
}

impl Player for Host {
    fn id(&self) -> Id {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl IsHost for Host {
    fn is_host(&self) -> bool {
        true
    }
}
