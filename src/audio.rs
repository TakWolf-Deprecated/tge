use crate::error::GameResult;

pub struct Audio {}

impl Audio {

    pub(crate) fn new(audio_config: AudioConfig) -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct AudioConfig {}

impl AudioConfig {

    pub fn new() -> Self {
        Self {}
    }

}
