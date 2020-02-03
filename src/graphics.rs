use crate::error::{GameError, GameResult};
use winit::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::rc::Rc;

pub struct Graphics {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
}

impl Graphics {

    pub(crate) fn new(graphics_config: GraphicsConfig, context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>) -> GameResult<Self> {
        Ok(Self {
            context_wrapper
        })
    }

    pub(crate) fn present(&mut self) -> GameResult {
        self.context_wrapper.swap_buffers()
            .map_err(|error| GameError::RuntimeError(format!("{}", error)))
    }

}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {}

impl GraphicsConfig {

    pub fn new() -> Self {
        Self {}
    }

}
