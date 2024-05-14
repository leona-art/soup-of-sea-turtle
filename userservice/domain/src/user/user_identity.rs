use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserIdError {
    #[error("不正なユーザIDです : {0}")]
    Invalid(String),
}

/// ユーザー識別子  
/// SOプロバイダーから取得した一意の識別子を使用する
pub struct UserIdentity {
    value: String,
}

impl UserIdentity {
    pub fn new(value:&str) -> Self {
        UserIdentity { value: value.to_string() }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl FromStr for UserIdentity {
    type Err = UserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ここでバリデーションを行う
        
        Ok(UserIdentity { value: s.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_invalid() {
        let user_id_str = "invalid";
        let user_id = user_id_str.parse::<UserIdentity>();
        assert_eq!(user_id.is_err(), true);
    }
}