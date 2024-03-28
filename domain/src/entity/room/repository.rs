use std::error::Error;

use super::Room;

pub trait RoomRepository {
    fn save(&self, room: Room) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, id: &str) -> Result<Option<Room>, Box<dyn Error>>;
    fn find_all(&self) -> Result<Vec<Room>, Box<dyn Error>>;
}