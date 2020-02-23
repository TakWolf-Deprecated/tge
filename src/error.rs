use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    IoError(Box<dyn Error>),
    InitError(Box<dyn Error>),
    StateError(Box<dyn Error>),
    RuntimeError(Box<dyn Error>),
    NotSupportedError(Box<dyn Error>),
}

impl Error for GameError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let source = match self {
            GameError::IoError(source) => source,
            GameError::InitError(source) => source,
            GameError::StateError(source) => source,
            GameError::RuntimeError(source) => source,
            GameError::NotSupportedError(source) => source,
        };
        Some(source.as_ref())
    }

}

impl fmt::Display for GameError {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::IoError(source) => write!(fmt, "GameError::IoError: {}", source),
            GameError::InitError(source) => write!(fmt, "GameError::InitError: {}", source),
            GameError::StateError(source) => write!(fmt, "GameError::StateError: {}", source),
            GameError::RuntimeError(source) => write!(fmt, "GameError::RuntimeError: {}", source),
            GameError::NotSupportedError(source) => write!(fmt, "GameError::NotSupportedError: {}", source),
        }
    }

}

pub type GameResult<T = ()> = Result<T, GameError>;
