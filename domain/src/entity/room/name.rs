use std::str::FromStr;

use thiserror::Error;

const MAX_ROOM_NAME_LENGTH: usize = 50;

#[derive(Error,Debug)]
pub enum RoomNameErorr{
    #[error("Room name is empty.")]
    Empty,
    #[error("Room name is too long. please use {} characters or less.", MAX_ROOM_NAME_LENGTH)]
    TooLong,
}

#[derive(Debug,Clone, Eq, PartialEq)]
pub struct RoomName {
    value: String,
} 

impl FromStr for RoomName {
    type Err = RoomNameErorr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(RoomNameErorr::Empty);
        }
        if s.len() > MAX_ROOM_NAME_LENGTH {
            return Err(RoomNameErorr::TooLong);
        }
        Ok(Self { value: s.to_string() })
    }
}

impl ToString for RoomName {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let name = RoomName::from_str("test").unwrap();
        assert_eq!(name.to_string(), "test");
    }
    #[test]
    fn test_new_error_empty() {
        let test_name = "";
        let name = RoomName::from_str(test_name);
        assert!(name.is_err());
    }
    #[test]
    fn test_new_error_too_long() {
        let test_name = "a".repeat(MAX_ROOM_NAME_LENGTH + 1);
        let name = RoomName::from_str(&test_name);
        assert!(name.is_err());
    }
}