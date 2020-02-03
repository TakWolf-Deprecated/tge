use crate::error::GameResult;

pub struct Keyboard {}

impl Keyboard {

    pub(crate) fn new(keyboard_config: KeyboardConfig) -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct KeyboardConfig {}

impl KeyboardConfig {

    pub fn new() -> Self {
        Self {}
    }

}
