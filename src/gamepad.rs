use crate::error::GameResult;

pub struct Gamepad {}

impl Gamepad {

    pub(crate) fn new() -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct GamepadConfig {}

impl GamepadConfig {

    pub fn new() -> Self {
        Self {}
    }

}
