use std::str::FromStr;

use thiserror::Error;

const MAX_NAME_LENGTH: usize = 20;
#[derive(Error, Debug)]
pub enum UserNameError {
    #[error("user name is empty")]
    Empty,
    #[error("user name is too long. please use {} characters or less.", MAX_NAME_LENGTH)]
    TooLong,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserName {
    value: String,
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
impl FromStr for UserName {
    type Err = UserNameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(UserNameError::Empty);
        }
        if s.len() > MAX_NAME_LENGTH {
            return Err(UserNameError::TooLong);
        }
        Ok(Self { value: s.to_string() })
    }
}
