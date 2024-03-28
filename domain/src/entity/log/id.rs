use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum LogIdError {
    #[error("{0} is not a valid uuid.")]
    InvailedValue(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LogId {
    pub id: Uuid,
}

impl LogId {
    pub fn new() -> Self {
        LogId { id: Uuid::new_v4() }
    }
}

impl FromStr for LogId {
    type Err = LogIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s).map_err(|_| LogIdError::InvailedValue(s.to_string()))?;
        Ok(LogId { id })
    }
}

impl ToString for LogId {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}