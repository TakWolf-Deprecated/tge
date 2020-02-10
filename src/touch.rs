use crate::error::GameResult;

pub struct Touch {}

impl Touch {

    pub(crate) fn new(touch_config: TouchConfig) -> GameResult<Self> {
        Ok(Self {})
    }

    pub(crate) fn reset_states(&mut self) {
        // TODO
    }

}

#[derive(Debug, Clone)]
pub struct TouchConfig {}

impl TouchConfig {

    pub fn new() -> Self {
        Self {}
    }

}
