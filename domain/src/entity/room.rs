pub mod id;
pub mod members;
pub mod name;
pub mod repository;
use std::str::FromStr;

use id::{RoomId, RoomIdError};
use name::{RoomName, RoomNameErorr};
use thiserror::Error;

use self::members::{Members, MembersError};

use super::user::User;

#[derive(Error, Debug)]
pub enum RoomError {
    #[error("Room id error: {0}")]
    RoomIdError(#[from] RoomIdError),
    #[error("Room name error: {0}")]
    RoomNameError(#[from] RoomNameErorr),
    #[error("Room members error: {0}")]
    RoomMembersError(#[from] MembersError),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Room {
    pub(self) id: RoomId,
    pub(self) name: RoomName,
    pub members: Members,
}

impl Room {
    pub fn new(name: &str) -> Result<Self, RoomError> {
        let name = RoomName::from_str(name)?;
        Ok(Self {
            id: RoomId::new(),
            name,
            members: Members::new(),
        })
    }
    pub fn id(&self) -> String {
        self.id.to_string()
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

impl TryFrom<(&str, &str, &[User])> for Room {
    type Error = RoomError;
    fn try_from(data: (&str, &str, &[User])) -> Result<Self, Self::Error> {
        let id = RoomId::from_str(data.0)?;
        let name = RoomName::from_str(data.1)?;
        let members = Members::try_from(data.2.to_vec())?;
        Ok(Self { id, name, members })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let room = Room::new("test").unwrap();
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
        let room = Room::new(&test_name);
        assert!(room.is_err());
    }
}
