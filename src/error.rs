use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    IoError(String),
    InitError(String),
    StateError(String),
    RuntimeError(String),
    NotSupportedError(String),
}

impl fmt::Display for GameError {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::IoError(message) => write!(fmt, "GameError::IoError: {}", message),
            GameError::InitError(message) => write!(fmt, "GameError::InitError: {}", message),
            GameError::StateError(message) => write!(fmt, "GameError::StateError: {}", message),
            GameError::RuntimeError(message) => write!(fmt, "GameError::RuntimeError: {}", message),
            GameError::NotSupportedError(message) => write!(fmt, "GameError::NotSupportedError: {}", message),
        }
    }

}

impl Error for GameError {}

pub type GameResult<T = ()> = Result<T, GameError>;
