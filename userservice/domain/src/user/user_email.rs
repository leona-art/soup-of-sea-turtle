use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserEmailError {
    #[error("メールアドレスが空です")]
    Empty,
    #[error("メールアドレスが不正です : {0}")]
    Invalid(String),
}

pub struct UserEmail {
    value: String,
}

impl UserEmail {
    pub fn new(value: &str) -> Result<Self, UserEmailError> {
        Self::validate(value)?;
        Ok(UserEmail {
            value: value.to_string(),
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(value: &str) -> Result<(), UserEmailError> {
        if value.is_empty() {
            return Err(UserEmailError::Empty);
        }
        if !value.contains('@') {
            return Err(UserEmailError::Invalid(value.to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user_email = UserEmail::new("test@test.com");
        assert_eq!(user_email.is_ok(), true);
    }
}
