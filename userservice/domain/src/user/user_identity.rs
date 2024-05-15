use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("不正なユーザIDです : {0}")]
    Invalid(String),
}

/// ユーザー識別子  
pub struct UserIdentity {
    value: String,
}

impl UserIdentity {
    pub fn new() -> Self {
        let value = uuid::Uuid::now_v7().to_string();
        UserIdentity { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FromStr for UserIdentity {
    type Err = UserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ここでバリデーションを行う
        let _ = uuid::Uuid::parse_str(s).map_err(|_| UserIdError::Invalid(s.to_string()))?;
        Ok(UserIdentity {
            value: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user_id = UserIdentity::new();
        assert_eq!(uuid::Uuid::parse_str(user_id.value()).is_ok(), true);
    }

    #[test]
    fn test_from_str() {
        let user_id_str = "00000000-0000-0000-0000-000000000000";
        let user_id = user_id_str.parse::<UserIdentity>();
        assert_eq!(user_id.is_ok(), true);
    }

    #[test]
    fn test_from_str_invalid() {
        let user_id_str = "invalid";
        let user_id = user_id_str.parse::<UserIdentity>();
        assert_eq!(user_id.is_err(), true);
    }
}
