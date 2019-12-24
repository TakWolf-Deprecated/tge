
#[derive(Debug)]
pub enum GameError {
    InitError(String),
    StateError(String),
    RuntimeError(String),
}

pub type GameResult<T = ()> = Result<T, GameError>;
