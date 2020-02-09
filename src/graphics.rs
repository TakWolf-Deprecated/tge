mod color;

pub use color::Color;

use crate::error::{GameError, GameResult};
use winit::window::Window;
use winit::dpi::PhysicalSize;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::{Context, HasContext};
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

    fn window(&self) -> &Window {
        self.context_wrapper.window()
    }

    pub(crate) fn gl(&self) -> &Rc<Context> {
        &self.gl
    }

    pub(crate) fn resize(&mut self, physical_size: PhysicalSize<u32>) {
        self.context_wrapper.resize(physical_size);
    }

    pub(crate) fn present(&mut self) -> GameResult {
        self.context_wrapper.swap_buffers()
            .map_err(|error| GameError::RuntimeError(format!("{}", error)))
    }

    pub(crate) fn clean(&mut self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
            self.gl.bind_vertex_array(None);
            self.gl.use_program(None);
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            self.gl.clear_color(color.red, color.green, color.blue, color.alpha);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {}

impl GraphicsConfig {

    pub fn new() -> Self {
        Self {}
    }

}
