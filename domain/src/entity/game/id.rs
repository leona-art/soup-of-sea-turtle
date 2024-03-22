use std::str::FromStr;

use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameIdError {
    #[error("{0} is not a valid uuid.")]
    InvailedValue(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameId {
    value: Uuid,
}

impl GameId{
    pub fn new() -> Self{
        Self{
            value: Uuid::new_v4()
        }
    }
}
impl ToString for GameId{
    fn to_string(&self) -> String{
        self.value.to_string()
    }
}

impl FromStr for GameId {
    type Err = GameIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(|value| Self { value })
            .map_err(|_| GameIdError::InvailedValue(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let id = GameId::new();
        assert_eq!(id.to_string().len(), 36);
    }
    #[test]
    fn test_from_string() {
        let id = GameId::from_str("00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");
    }
    #[test]
    fn test_from_string_error() {
        let id = GameId::from_str("00000000-0000-0000-0000-00000000000");
        assert!(id.is_err());
    }
}