use domain::entity::user::{repository::UserRepository, User, UserError};
use infra::in_memory::InMemoryUserRepository;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("User id is invalid")]
    UserError(#[from] UserError),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("DB error")]
    DBError(#[from] Box<dyn Error>),
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
}
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            name: user.name(),
        }
    }
}

pub struct CreateUserService<R>
where
    R: UserRepository,
{
    pub repository: Arc<R>,
}

impl<R> CreateUserService<R>
where
    R: UserRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub fn create_user(&self, name: &str) -> Result<UserDto, CreateUserError> {
        let user = User::new(name)?;
        self.repository.save(user.clone())?;
        Ok(user.into())
    }
}

#[derive(Error, Debug)]
pub enum GetUserError {
    #[error("User not found")]
    UserNotFound,
    #[error("DB error")]
    DBError(#[from] Box<dyn Error>),
}
pub struct GetUserService<R>
where
    R: UserRepository,
{
    pub repository: Arc<R>,
}
impl GetUserService<InMemoryUserRepository> {
    pub fn new(repository: Arc<InMemoryUserRepository>) -> Self {
        Self { repository }
    }
    pub fn get_user(&self, id: &str) -> Result<UserDto, GetUserError> {
        let Some(user) = self.repository.find_by_id(id)? else{
            return Err(GetUserError::UserNotFound);
        };
        Ok(user.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infra::in_memory::InMemoryUserRepository;

    #[test]
    fn test_create_user() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let service = CreateUserService::new(repository.clone());
        let result = service.create_user("Alice").unwrap();
        println!("{:?}", result);
    }
}
