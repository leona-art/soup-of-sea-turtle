use thiserror::Error;

#[derive(Error, Debug)]
pub enum PartyError{
    #[error("パーティーのロールが不正です。")]
    InvalidMembersRole,
    #[error("パーティーのメンバー数が不正です。")]
    InvalidMembersCount,
}