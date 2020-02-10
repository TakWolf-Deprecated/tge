use crate::error::GameResult;

pub struct Audio {}

impl Audio {

    pub(crate) fn new(audio_config: AudioConfig) -> GameResult<Self> {
        Ok(Self {})
    }

    pub(crate) fn suspend(&mut self) {
        // TODO
    }

    pub(crate) fn resume(&mut self) {
        // TODO
    }

}

#[derive(Debug, Clone)]
pub struct AudioConfig {}

impl AudioConfig {

    pub fn new() -> Self {
        Self {}
    }

}
