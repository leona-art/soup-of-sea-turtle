use std::error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid state transition.")]
    InvalidState,
}