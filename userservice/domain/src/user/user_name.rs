use thiserror::Error;

const MAX_USERNAME_LENGTH: usize = 32;

#[derive(Debug, Error)]
pub enum UserNameError {
    #[error("ユーザ名が空です")]
    Empty,
    #[error("ユーザ名が長すぎます{0}字以内で入力してください")]
    TooLong(usize),

}

pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(value: &str) -> Result<Self, UserNameError> {
        if value.is_empty() {
            return Err(UserNameError::Empty);
        }
        if value.len() > MAX_USERNAME_LENGTH {
            return Err(UserNameError::TooLong(MAX_USERNAME_LENGTH));
        }
        Ok(UserName { value: value.to_string()})
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}