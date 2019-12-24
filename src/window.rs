use crate::error::GameResult;

pub struct Window {}

impl Window {

    pub(crate) fn new() -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct WindowConfig {}

impl WindowConfig {

    pub fn new() -> Self {
        Self {}
    }

}
