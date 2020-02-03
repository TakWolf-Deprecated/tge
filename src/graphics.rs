use crate::error::{GameError, GameResult};
use winit::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::Context;
use std::rc::Rc;

pub struct Graphics {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
    gl: Rc<Context>,
}

impl Graphics {

    pub(crate) fn new(graphics_config: GraphicsConfig, context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>) -> GameResult<Self> {
        let gl = Context::from_loader_function(|symbol| context_wrapper.get_proc_address(symbol).cast());
        Ok(Self {
            context_wrapper,
            gl: Rc::new(gl),
        })
    }

    pub(crate) fn gl(&self) -> &Rc<Context> {
        &self.gl
    }

    pub(crate) fn prepare(&mut self) -> GameResult {
        Ok(())
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
