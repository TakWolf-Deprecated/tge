use crate::error::{GameError, GameResult};

pub struct Filesystem {}

impl Filesystem {

    pub(crate) fn new(filesystem_config: FilesystemConfig) -> GameResult<Self> {
        Ok(Self {})
    }

}

#[derive(Debug, Clone)]
pub struct FilesystemConfig {}

impl FilesystemConfig {

    pub fn new() -> Self {
        Self {}
    }

}
