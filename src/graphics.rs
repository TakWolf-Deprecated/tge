use crate::error::GameResult;

pub struct Graphics {}

impl Graphics {

    pub(crate) fn new() -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {}

impl GraphicsConfig {

    pub fn new() -> Self {
        Self {}
    }

}
