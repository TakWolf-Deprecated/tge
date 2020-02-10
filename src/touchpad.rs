use crate::error::GameResult;

pub struct Touchpad {}

impl Touchpad {

    pub(crate) fn new(touchpad_config: TouchpadConfig) -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct TouchpadConfig {}

impl TouchpadConfig {

    pub fn new() -> Self {
        Self {}
    }

}
