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
            Self::IoError(source) => source,
            Self::InitError(source) => source,
            Self::StateError(source) => source,
            Self::RuntimeError(source) => source,
            Self::NotSupportedError(source) => source,
        };
        Some(source.as_ref())
    }

}

impl fmt::Display for GameError {

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(source) => write!(fmt, "GameError::IoError: {}", source),
            Self::InitError(source) => write!(fmt, "GameError::InitError: {}", source),
            Self::StateError(source) => write!(fmt, "GameError::StateError: {}", source),
            Self::RuntimeError(source) => write!(fmt, "GameError::RuntimeError: {}", source),
            Self::NotSupportedError(source) => write!(fmt, "GameError::NotSupportedError: {}", source),
        }
    }

}

pub type GameResult<T = ()> = Result<T, GameError>;
