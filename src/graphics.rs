#[allow(dead_code)]
mod opengl;
mod program;
mod color;
mod vertex;
#[allow(dead_code)]
mod renderer;
mod image;
mod texture;
mod canvas;
mod command;

use opengl::BufferUsage;
use renderer::{Renderer, RendererBuilder};
use command::DrawCommand;

pub use opengl::{PrimitiveType, FilterMode, Filter, WrapMode, Wrap};
pub use program::Program;
pub use color::Color;
pub use vertex::Vertex;
pub use self::image::{Image, validate_pixels};
pub use texture::{Texture, TextureHolder};
pub use canvas::Canvas;
pub use command::SpriteDrawParams;

use crate::error::{GameError, GameResult};
use crate::math::{Position, Point, Size, Region, Viewport};
use winit::window::Window;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::{Context, HasContext};
use glam::{Vec3, Vec4, Quat, Mat4};
use std::rc::Rc;

const SPRITE_VERTEX_COUNT: usize = 4;
const SPRITE_ELEMENT_COUNT: usize = 6;
const SPRITE_ELEMENTS: [u32; SPRITE_ELEMENT_COUNT] = [
    0, 2, 1,
    1, 2, 3
];

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
    default_texture: Texture,
    renderer: Renderer,
    vertices: Vec<Vertex>,
    elements: Vec<u32>,
    draw_command: DrawCommand,
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

        let default_texture = Texture::white_1_x_1(gl.clone())?;

        let renderer = RendererBuilder::new(gl.clone())?
            .init_vertex_size(BufferUsage::Stream, graphics_config.renderer_vertex_size)
            .init_element_size(BufferUsage::Stream, graphics_config.renderer_element_size)
            .build()?;
        let vertices = Vec::with_capacity(graphics_config.renderer_vertex_size);
        let elements = Vec::with_capacity(graphics_config.renderer_element_size);

        let draw_command = DrawCommand {
            texture: default_texture.texture().clone(),
            primitive: PrimitiveType::Triangles,
        };

        unsafe {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

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
            default_texture,
            renderer,
            vertices,
            elements,
            draw_command,
        })
    }

    fn window(&self) -> &Window {
        self.context_wrapper.window()
    }

    pub(crate) fn gl(&self) -> &Rc<Context> {
        &self.gl
    }

    pub(crate) fn resize(&mut self, scale_factor: f64, physical_size: PhysicalSize<u32>) {
        self.context_wrapper.resize(physical_size);
        if self.current_canvas.is_none() {
            let logical_size = physical_size.to_logical::<f32>(scale_factor);
            self.size.set(logical_size.width, logical_size.height);
            self.viewport.set(0.0, 0.0, logical_size.width, logical_size.height);
            unsafe {
                self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
            }
            self.projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
            self.current_program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn flush(&mut self) {
        if !self.vertices.is_empty() && !self.elements.is_empty() {
            self.renderer.update_vertices(0, &self.vertices);
            self.renderer.update_elements(0, &self.elements).expect("renderer update elements error");
            self.draw_command.texture.bind();
            self.renderer.draw_elements(self.draw_command.primitive, self.elements.len(), 0);
            self.draw_command.texture.unbind();
        }
        self.vertices.clear();
        self.elements.clear();
    }

    pub(crate) fn present(&mut self) -> GameResult {
        self.flush();
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
        let viewport = viewport.map(|viewport| viewport.into())
            .unwrap_or_else(|| Viewport::new(0.0, 0.0, self.size.width, self.size.height));
        if self.viewport != viewport {
            self.flush();
            self.viewport = viewport;
            if self.current_canvas.is_some() {
                unsafe {
                    self.gl.viewport(
                        self.viewport.x.round() as i32,
                        self.viewport.y.round() as i32,
                        self.viewport.width.round() as i32,
                        self.viewport.height.round() as i32,
                    );
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, self.viewport.width, 0.0, self.viewport.height, -1.0, 1.0);
            } else {
                let scale_factor = self.window().scale_factor();
                let physical_viewport = {
                    let physical_position = LogicalPosition::new(self.viewport.x, self.viewport.y).to_physical::<i32>(scale_factor);
                    let physical_size = LogicalSize::new(self.viewport.width, self.viewport.height).to_physical::<i32>(scale_factor);
                    Viewport::new(physical_position.x, physical_position.y, physical_size.width, physical_size.height)
                };
                let physical_size = {
                    let physical_size = LogicalSize::new(self.size.width, self.size.height).to_physical::<i32>(scale_factor);
                    Size::new(physical_size.width, physical_size.height)
                };
                unsafe {
                    self.gl.viewport(
                        physical_viewport.x,
                        physical_size.height - physical_viewport.y - physical_viewport.height,
                        physical_viewport.width,
                        physical_viewport.height,
                    );
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, self.viewport.width, self.viewport.height, 0.0, -1.0, 1.0);
            }
            self.current_program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn use_program(&mut self, program: Option<&Program>) {
        let program = program.map(|program| program.program().clone())
            .unwrap_or_else(|| self.default_program.clone());
        if self.current_program != program {
            self.flush();
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
            self.flush();
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

    pub fn clear<C: Into<Color>>(&mut self, color: C) {
        let color = color.into();
        unsafe {
            self.gl.clear_color(color.red, color.green, color.blue, color.alpha);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn switch_draw_command(&mut self, draw_command: DrawCommand) {
        if self.draw_command != draw_command {
            self.flush();
            self.draw_command = draw_command;
        }
    }

    fn append_vertices_and_elements(&mut self, vertices: Vec<Vertex>, elements: Option<Vec<u32>>) {
        let mut elements = elements.unwrap_or_else(|| {
            let mut elements = Vec::with_capacity(vertices.len());
            for i in 0..vertices.len() as u32 {
                elements.push(i);
            }
            elements
        });

        let renderer_vertex_size = self.renderer.vertex_size();
        let renderer_element_size = self.renderer.element_size().unwrap_or(0);
        if renderer_vertex_size - self.vertices.len() < vertices.len() || renderer_element_size - self.elements.len() < elements.len() {
            self.flush();
        }
        assert!(renderer_vertex_size >= vertices.len(), "no enough renderer vertex size ({}): expect {}", renderer_vertex_size, vertices.len());
        assert!(renderer_element_size >= elements.len(), "no enough renderer element size ({}): expect {}", renderer_element_size, elements.len());

        let element_offset = self.vertices.len() as u32;
        self.vertices.extend(vertices);
        for element in elements.iter_mut() {
            *element += element_offset;
        }
        self.elements.extend(elements);
    }

    pub fn draw_vertices(&mut self, texture: Option<&dyn TextureHolder>, primitive: PrimitiveType, vertices: Vec<Vertex>, elements: Option<Vec<u32>>) {
        let texture = texture.map(|texture| texture.texture().clone())
            .unwrap_or_else(|| self.default_texture.texture().clone());

        self.switch_draw_command(DrawCommand {
            texture,
            primitive,
        });

        self.append_vertices_and_elements(vertices, elements);
    }

    pub fn draw_sprite(&mut self, texture: Option<&dyn TextureHolder>, params: SpriteDrawParams) {
        let (texture, texture_size) = match texture {
            Some(texture) => (texture.texture().clone(), texture.size()),
            None => (self.default_texture.texture().clone(), self.default_texture.size()),
        };

        self.switch_draw_command(DrawCommand {
            texture,
            primitive: PrimitiveType::Triangles,
        });

        let texture_size = Size::new(texture_size.width as f32, texture_size.height as f32);
        let source = params.source.unwrap_or_else(|| Region::new(0.0, 0.0, texture_size.width, texture_size.height));
        let origin = params.origin.unwrap_or_else(|| Point::zero());
        let position = params.position.map(|position| Vec3::new(position.x, position.y, 0.0)).unwrap_or_else(|| Vec3::zero());
        let rotation = params.rotation.map(|angle| Quat::from_rotation_z(angle.radians_value())).unwrap_or_else(|| Quat::from_rotation_z(0.0));
        let scale = params.scale.map(|scale| Vec3::new(scale.x, scale.y, 1.0)).unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0));
        let uv = Region::new(
            source.x / texture_size.width,
            source.y / texture_size.height,
            source.width / texture_size.width,
            source.height / texture_size.height,
        );
        let colors = params.colors.unwrap_or_else(|| [Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE]);

        let model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, position);

        let x0y0 = model_matrix * Vec4::new(-origin.x, -origin.y, 0.0, 1.0);
        let x1y0 = model_matrix * Vec4::new(-origin.x + source.width, -origin.y, 0.0, 1.0);
        let x0y1 = model_matrix * Vec4::new(-origin.x, -origin.y + source.height, 0.0, 1.0);
        let x1y1 = model_matrix * Vec4::new(-origin.x + source.width, -origin.y + source.height, 0.0, 1.0);

        let vertices = vec![
            Vertex {
                position: Position::new(x0y0.x(), x0y0.y()),
                uv: uv.top_left(),
                color: colors[0],
            },
            Vertex {
                position: Position::new(x1y0.x(), x1y0.y()),
                uv: uv.top_right(),
                color: colors[1],
            },
            Vertex {
                position: Position::new(x0y1.x(), x0y1.y()),
                uv: uv.bottom_left(),
                color: colors[2],
            },
            Vertex {
                position: Position::new(x1y1.x(), x1y1.y()),
                uv: uv.bottom_right(),
                color: colors[3],
            },
        ];
        let elements = SPRITE_ELEMENTS.to_vec();

        self.append_vertices_and_elements(vertices, Some(elements));
    }

}

#[derive(Debug, Clone)]
pub struct GraphicsConfig {
    default_filter: Filter,
    default_wrap: Wrap,
    renderer_vertex_size: usize,
    renderer_element_size: usize,
}

impl GraphicsConfig {

    pub fn new() -> Self {
        Self {
            default_filter: Filter::default(),
            default_wrap: Wrap::default(),
            renderer_vertex_size: SPRITE_VERTEX_COUNT * 1024,
            renderer_element_size: SPRITE_ELEMENT_COUNT * 1024,
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

    pub fn renderer_vertex_size(mut self, size: usize) -> Self {
        self.renderer_vertex_size = size;
        self
    }

    pub fn renderer_element_size(mut self, size: usize) -> Self {
        self.renderer_element_size = size;
        self
    }

    pub fn renderer_sprite_size(mut self, size: usize) -> Self {
        self.renderer_vertex_size = SPRITE_VERTEX_COUNT * size;
        self.renderer_element_size = SPRITE_ELEMENT_COUNT * size;
        self
    }

}
