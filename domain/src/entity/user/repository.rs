use std::error::Error;

use super::User;

pub trait UserRepository {
    fn save(&self, user: User) -> Result<(),Box<dyn Error>>;
    fn find_by_id(&self, id: &str) -> Result<Option<User>,Box<dyn Error>>;
}