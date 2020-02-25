use crate::error::GameResult;

pub struct Audio {}

impl Audio {

    pub(crate) fn new(_: AudioConfig) -> GameResult<Self> {
        Ok(Self {})
    }

    pub(crate) fn suspend(&mut self) {}

    pub(crate) fn resume(&mut self) {}

}

#[derive(Debug, Clone)]
pub struct AudioConfig {}

impl AudioConfig {

    pub fn new() -> Self {
        Self {}
    }

}
