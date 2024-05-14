use super::User;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("ユーザが見つかりません")]
    NotFound,
    #[error("ユーザが既に存在します")]
    AlreadyExists,
    #[error("コネクトエラー: {0}")]
    ConnectError(String),
    #[error("データベースエラー: {0}")]
    DatabaseError(String),
}

pub trait UserRepository {
    fn find_by_id(&self, id: &str) -> Option<User>;
    fn save(&self, user: User) -> Result<(), String>;
    fn delete(&self, user: User) -> Result<(), String>;
}