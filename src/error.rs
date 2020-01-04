use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    InitError(String),
    StateError(String),
    RuntimeError(String),
    Unsupported(String),
}

impl fmt::Display for GameError {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InitError(message) => write!(fmt, "game init error: {}", message),
            GameError::StateError(message) => write!(fmt, "game state error: {}", message),
            GameError::RuntimeError(message) => write!(fmt, "game runtime error: {}", message),
            GameError::Unsupported(message) => write!(fmt, "unsupported operation: {}", message),
        }
    }

}

impl Error for GameError {}

pub type GameResult<T = ()> = Result<T, GameError>;
