pub mod user_identity;
pub mod user_name;
pub mod repository;

use user_identity::UserIdentity;
use user_name::UserName;

pub struct User {
    id: user_identity::UserIdentity,
    name: user_name::UserName,
}

impl User {
    pub fn new(name: &str) -> Result<Self, user_name::UserNameError> {
        let id = UserIdentity::new();
        let name = user_name::UserName::new(name)?;
        Ok(User { id, name })
    }

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn name(&self) -> &str {
        self.name.value()
    }
}