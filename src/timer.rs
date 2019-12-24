use crate::error::GameResult;

pub struct Timer {}

impl Timer {

    pub(crate) fn new() -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct TimerConfig {}

impl TimerConfig {

    pub fn new() -> Self {
        Self {}
    }

}
