pub mod repository;
pub mod user_email;
pub mod user_identity;
pub mod user_name;

use user_email::{UserEmail, UserEmailError};
use user_identity::UserIdentity;
use user_name::{UserName, UserNameError};

use thiserror::Error;
#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    UserNameError(#[from] UserNameError),
    #[error("{0}")]
    UserEmailError(#[from] UserEmailError),
}

pub struct User {
    id: user_identity::UserIdentity,
    name: UserName,
    email: UserEmail,
}

impl User {
    pub fn new(name: &str, email: &str) -> Result<Self, UserError> {
        let id = UserIdentity::new();
        let name = UserName::new(name)?;
        let email = UserEmail::new(email)?;
        Ok(User { id, name, email })
    }

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }
    pub fn email(&self) -> &str {
        self.email.value()
    }
}
