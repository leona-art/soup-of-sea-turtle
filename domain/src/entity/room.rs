pub mod id;
pub mod name;
pub mod members;
use std::str::FromStr;

use id::{RoomId, RoomIdError};
use name::{RoomName,RoomNameErorr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoomError {
    #[error("Room id error: {0}")]
    RoomIdError(#[from] RoomIdError),
    #[error("Room name error: {0}")]
    RoomNameError(#[from] RoomNameErorr),
}



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Room {
    id: RoomId,
    name: RoomName,
}

impl Room {
    pub fn new(name: &str) -> Result<Self, RoomError> {
        let name = RoomName::from_str(name)?;
        Ok(Self {
            id: RoomId::new(),
            name,
        })
    }
    pub fn id(&self) -> &RoomId {
        &self.id
    }
    pub fn name(&self) -> &RoomName {
        &self.name
    }
}

impl TryFrom<&str> for Room {
    type Error = RoomError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let name = RoomName::from_str(value)?;
        Ok(Self {
            id: RoomId::new(),
            name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let room = Room::try_from("test").unwrap();
        assert_eq!(room.name().to_string(), "test");
    }
    #[test]
    fn test_new_error_empty() {
        let test_name = "";
        let room = Room::new(test_name);
        assert!(room.is_err());
    }
    #[test]
    fn test_new_error_too_long() {
        let test_name = "a".repeat(51);
        let room = Room::try_from(&test_name[..]);
        assert!(room.is_err());
    }
}