mod opengl;
mod color;
mod program;
mod texture;
mod canvas;

pub use opengl::{FilterMode, Filter, WrapMode, Wrap};
pub use color::Color;
pub use program::Program;
pub use texture::Texture;
pub use canvas::Canvas;

use texture::TextureHolder;
use crate::error::{GameError, GameResult};
use crate::math::{Size, Viewport};
use winit::window::Window;
use winit::dpi::PhysicalSize;
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::{Context, HasContext};
use glam::Mat4;
use std::rc::Rc;

pub struct Graphics {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
    gl: Rc<Context>,
    size: Size,
    viewport: Viewport,
    projection_matrix: Mat4,
    default_program: Rc<opengl::Program>,
    current_program: Rc<opengl::Program>,
    current_canvas: Option<Rc<opengl::Framebuffer>>,
    default_filter: Filter,
    default_wrap: Wrap,
}

impl Graphics {

    pub(crate) fn new(graphics_config: GraphicsConfig, context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>) -> GameResult<Self> {
        let gl = Context::from_loader_function(|symbol| context_wrapper.get_proc_address(symbol).cast());
        let gl = Rc::new(gl);

        let physical_size = context_wrapper.window().inner_size();
        let logical_size = physical_size.to_logical::<f32>(context_wrapper.window().scale_factor());
        let size = Size::new(logical_size.width, logical_size.height);
        let viewport = Viewport::new(0.0, 0.0, logical_size.width, logical_size.height);
        unsafe {
            gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
        }
        let projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);

        let default_program = Program::default(gl.clone())?;
        let current_program = default_program.clone();
        current_program.bind();
        current_program.set_uniform_matrix_4("u_projection", &projection_matrix.to_cols_array());

        Ok(Self {
            context_wrapper,
            gl,
            size,
            viewport,
            projection_matrix,
            default_program,
            current_program,
            current_canvas: None,
            default_filter: graphics_config.default_filter,
            default_wrap: graphics_config.default_wrap,
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
        if self.current_canvas.is_none() {
            let logical_size = physical_size.to_logical::<f32>(self.window().scale_factor());
            self.size.set(logical_size.width, logical_size.height);
            self.viewport.set(0.0, 0.0, logical_size.width, logical_size.height);
            unsafe {
                self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
            }
            self.projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
            self.current_program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub(crate) fn present(&mut self) -> GameResult {
        self.context_wrapper.swap_buffers()
            .map_err(|error| GameError::RuntimeError(Box::new(error)))
    }

    pub(crate) fn clean(&mut self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
            self.gl.bind_vertex_array(None);
            self.gl.use_program(None);
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn set_viewport<V: Into<Viewport>>(&mut self, viewport: Option<V>) {
        //self.viewport = viewport.unwrap_or_else(|| Viewport::new(0.0, 0.0, self.size.width, self.size.height)).into();
        // TODO


    }

    pub fn use_program(&mut self, program: Option<&Program>) {
        let program = program.map(|program| program.program().clone())
            .unwrap_or_else(|| self.default_program.clone());
        if self.current_program != program {
            self.current_program = program;
            self.current_program.bind();
            self.current_program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn set_canvas(&mut self, canvas: Option<&Canvas>) {
        let (canvas, canvas_size) = match canvas {
            Some(canvas) => (Some(canvas.framebuffer().clone()), Some(canvas.size())),
            None => (None, None),
        };
        if self.current_canvas != canvas {
            if canvas.is_none() {
                if let Some(canvas) = &self.current_canvas {
                    canvas.unbind();
                }
            }
            self.current_canvas = canvas;
            if let Some(canvas) = &self.current_canvas {
                canvas.bind();
            }
            if let Some(canvas_size) = canvas_size {
                self.size.set(canvas_size.width as f32, canvas_size.height as f32);
                self.viewport.set(0.0, 0.0, self.size.width, self.size.height);
                unsafe {
                    self.gl.viewport(0, 0, canvas_size.width as i32, canvas_size.height as i32);
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, self.size.width, 0.0, self.size.height, -1.0, 1.0);
            } else {
                let physical_size = self.window().inner_size();
                let logical_size = physical_size.to_logical::<f32>(self.window().scale_factor());
                self.size.set(logical_size.width, logical_size.height);
                self.viewport.set(0.0, 0.0, logical_size.width, logical_size.height);
                unsafe {
                    self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
            }
            self.current_program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn default_filter(&self) -> Filter {
        self.default_filter
    }

    pub fn set_default_filter(&mut self, filter: Filter) {
        self.default_filter = filter;
    }

    pub fn default_wrap(&self) -> Wrap {
        self.default_wrap
    }

    pub fn set_default_wrap(&mut self, wrap: Wrap) {
        self.default_wrap = wrap;
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            self.gl.clear_color(color.red, color.green, color.blue, color.alpha);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    default_filter: Filter,
    default_wrap: Wrap,
}

impl GraphicsConfig {

    pub fn new() -> Self {
        Self {
            default_filter: Filter::default(),
            default_wrap: Wrap::default(),
        }
    }

    pub fn default_filter(mut self, filter: Filter) -> Self {
        self.default_filter = filter;
        self
    }

    pub fn default_wrap(mut self, wrap: Wrap) -> Self {
        self.default_wrap = wrap;
        self
    }

}
