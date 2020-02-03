use crate::error::GameResult;

pub struct Mouse {}

impl Mouse {

    pub(crate) fn new(mouse_config: MouseConfig) -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct MouseConfig {}

impl MouseConfig {

    pub fn new() -> Self {
        Self {}
    }

}
