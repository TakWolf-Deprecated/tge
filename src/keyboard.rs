mod code;

pub use code::KeyCode;

use crate::error::GameResult;

pub struct Keyboard {}

impl Keyboard {

    pub(crate) fn new(keyboard_config: KeyboardConfig) -> GameResult<Self> {
        Ok(Self {})
    }

    pub(crate) fn reset_states(&mut self) {
        // TODO
    }

}

#[derive(Debug, Clone)]
pub struct KeyboardConfig {}

impl KeyboardConfig {

    pub fn new() -> Self {
        Self {}
    }

}
