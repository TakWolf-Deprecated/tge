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
mod font;
mod texture_ref;
mod params;

use opengl::BufferUsage;
use renderer::{Renderer, RendererBuilder};

pub use opengl::{PrimitiveType, FilterMode, Filter, WrapMode, Wrap};
pub use program::Program;
pub use color::Color;
pub use vertex::Vertex;
pub use self::image::Image;
pub(crate) use self::image::validate_pixels;
pub use texture::Texture;
pub use canvas::Canvas;
pub use font::Font;
pub use texture_ref::TextureRef;
pub use params::{MeshDrawParams, SpriteDrawParams, TextLayoutGravity, TextDrawParams};

use crate::error::{GameError, GameResult};
use crate::math::{Position, Size, Region, Viewport, Transform};
use winit::window::Window;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use glutin::{ContextWrapper, PossiblyCurrent};
use glow::{Context, HasContext};
use glam::{Vec4, Mat4};
use std::rc::Rc;

const SPRITE_VERTEX_COUNT: usize = 4;
const SPRITE_ELEMENT_COUNT: usize = 6;
const SPRITE_ELEMENTS: [u16; SPRITE_ELEMENT_COUNT] = [
    0, 2, 1,
    1, 2, 3
];

#[derive(PartialEq)]
struct DrawCommand {
    pub texture: Rc<opengl::Texture>,
    pub primitive: PrimitiveType,
}

pub struct Graphics {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
    gl: Rc<Context>,
    size: Size,
    viewport: Viewport,
    projection_matrix: Mat4,
    transform_matrix: Mat4,
    transform_stack: Vec<Mat4>,
    default_program: Rc<opengl::Program>,
    program: Rc<opengl::Program>,
    default_filter: Filter,
    default_wrap: Wrap,
    default_texture: Rc<opengl::Texture>,
    canvas: Option<Rc<opengl::Framebuffer>>,
    max_texture_size: u32,
    renderer: Renderer,
    vertices: Vec<Vertex>,
    elements: Vec<u16>,
    draw_command: DrawCommand,
}

impl Graphics {
    pub(crate) fn new(graphics_config: GraphicsConfig, context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>, gl: Rc<Context>) -> GameResult<Self> {
        let physical_size = context_wrapper.window().inner_size();
        let scale_factor = context_wrapper.window().scale_factor();
        let logical_size = physical_size.to_logical(scale_factor);
        let size = Size::new(logical_size.width, logical_size.height);
        let viewport = Viewport::new(0.0, 0.0, logical_size.width, logical_size.height);
        unsafe {
            gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
        }
        let projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
        let transform_matrix = Mat4::identity();
        let transform_stack = Vec::new();

        let default_program = Program::default(gl.clone())?;
        let program = default_program.clone();
        program.bind();
        program.set_uniform_matrix_4("u_projection", &projection_matrix.to_cols_array());

        let default_texture = Texture::default(gl.clone())?;

        let max_texture_size = unsafe {
            gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) as u32
        };

        let renderer = RendererBuilder::new(gl.clone())?
            .init_vertex_size(BufferUsage::Stream, graphics_config.renderer_vertex_size)
            .init_element_size(BufferUsage::Stream, graphics_config.renderer_element_size)
            .build()?;
        let vertices = Vec::with_capacity(graphics_config.renderer_vertex_size);
        let elements = Vec::with_capacity(graphics_config.renderer_element_size);

        let draw_command = DrawCommand {
            texture: default_texture.clone(),
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
            transform_matrix,
            transform_stack,
            default_program,
            program,
            default_filter: graphics_config.default_filter,
            default_wrap: graphics_config.default_wrap,
            default_texture,
            canvas: None,
            max_texture_size,
            renderer,
            vertices,
            elements,
            draw_command,
        })
    }

    pub(crate) fn resize(&mut self, physical_size: PhysicalSize<u32>, scale_factor: f64) {
        self.context_wrapper.resize(physical_size);
        if self.canvas.is_none() {
            let logical_size = physical_size.to_logical(scale_factor);
            self.size = Size::new(logical_size.width, logical_size.height);
            self.viewport = Viewport::new(0.0, 0.0, logical_size.width, logical_size.height);
            unsafe {
                self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
            }
            self.projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
            self.program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn flush(&mut self) {
        if !self.vertices.is_empty() && !self.elements.is_empty() {
            self.renderer.update_vertices(0, &self.vertices);
            self.renderer.update_elements(0, &self.elements);
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

    fn window(&self) -> &Window {
        self.context_wrapper.window()
    }

    pub(crate) fn gl(&self) -> Rc<Context> {
        self.gl.clone()
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    pub fn set_viewport(&mut self, viewport: Option<impl Into<Viewport>>) {
        let viewport = viewport.map(|viewport| viewport.into())
            .unwrap_or_else(|| Viewport::new(0.0, 0.0, self.size.width, self.size.height));
        if self.viewport != viewport {
            self.flush();
            self.viewport = viewport;
            if self.canvas.is_some() {
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
            self.program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn set_transform(&mut self, transform: impl Into<Transform>) {
        self.transform_matrix = transform.into().0;
    }

    pub fn apply_transform(&mut self, transform: impl Into<Transform>) {
        self.transform_matrix = transform.into().0 * self.transform_matrix;
    }

    pub fn push_transform(&mut self) {
        self.transform_stack.push(self.transform_matrix);
    }

    pub fn pop_transform(&mut self) {
        if let Some(matrix) = self.transform_stack.pop() {
            self.transform_matrix = matrix;
        }
    }

    pub fn use_program(&mut self, program: Option<&Program>) {
        let program = program.map(|program| program.program().clone())
            .unwrap_or_else(|| self.default_program.clone());
        if self.program != program {
            self.flush();
            self.program = program;
            self.program.bind();
            self.program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
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

    pub fn set_canvas(&mut self, canvas: Option<&Canvas>) {
        let (canvas, canvas_size) = match canvas {
            Some(canvas) => (Some(canvas.framebuffer().clone()), Some(canvas.size())),
            None => (None, None),
        };
        if self.canvas != canvas {
            self.flush();
            if canvas.is_none() {
                if let Some(canvas) = &self.canvas {
                    canvas.unbind();
                }
            }
            self.canvas = canvas;
            if let Some(canvas) = &self.canvas {
                canvas.bind();
            }
            if let Some(canvas_size) = canvas_size {
                self.size = Size::new(canvas_size.width as f32, canvas_size.height as f32);
                self.viewport = Viewport::new(0.0, 0.0, self.size.width, self.size.height);
                unsafe {
                    self.gl.viewport(0, 0, canvas_size.width as i32, canvas_size.height as i32);
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, self.size.width, 0.0, self.size.height, -1.0, 1.0);
            } else {
                let physical_size = self.window().inner_size();
                let scale_factor = self.window().scale_factor();
                let logical_size = physical_size.to_logical(scale_factor);
                self.size = Size::new(logical_size.width, logical_size.height);
                self.viewport = Viewport::new(0.0, 0.0, logical_size.width, logical_size.height);
                unsafe {
                    self.gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                }
                self.projection_matrix = Mat4::orthographic_rh_gl(0.0, logical_size.width, logical_size.height, 0.0, -1.0, 1.0);
            }
            self.program.set_uniform_matrix_4("u_projection", &self.projection_matrix.to_cols_array());
        }
    }

    pub fn clear(&mut self, color: impl Into<Color>) {
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

    fn append_vertices_and_elements(&mut self, vertices: Vec<Vertex>, elements: Option<Vec<u16>>) {
        let mut elements = elements.unwrap_or_else(|| (0..vertices.len() as u16).collect());
        if self.renderer.vertex_size() < self.vertices.len() + vertices.len() || self.renderer.element_size() < self.elements.len() + elements.len() {
            self.flush();
        }
        assert!(self.renderer.vertex_size() >= self.vertices.len() + vertices.len(), "no enough renderer vertex size");
        assert!(self.renderer.element_size() >= self.elements.len() + elements.len(), "no enough renderer element size");
        let append_vertex_count = vertices.len() as u16;
        let element_offset = self.vertices.len() as u16;
        for element in &mut elements {
            assert!(*element < append_vertex_count, "element must < append vertex count");
            *element += element_offset;
        }
        self.vertices.extend(vertices);
        self.elements.extend(elements);
    }

    pub fn draw_mesh<'a>(&mut self, texture: impl Into<TextureRef<'a>>, params: impl Into<Option<MeshDrawParams>>, transform: impl Into<Option<Transform>>) {
        let texture = texture.into();
        let params = params.into().unwrap_or_default();
        let transform = transform.into().unwrap_or_default();

        self.switch_draw_command(DrawCommand {
            texture: texture.texture().unwrap_or_else(|| self.default_texture.clone()),
            primitive: params.primitive.unwrap_or(PrimitiveType::Triangles),
        });

        let matrix = self.transform_matrix * transform.0;

        let vertices = params.vertices.map(|mut vertices| {
            for vertex in &mut vertices {
                vertex.position = {
                    let position = matrix * Vec4::new(vertex.position.x, vertex.position.y, 0.0, 1.0);
                    Position::new(position.x(), position.y())
                };
            }
            vertices
        }).unwrap_or_else(|| Vec::new());
        let elements = params.elements;
        self.append_vertices_and_elements(vertices, elements);
    }

    pub fn draw_sprite<'a>(&mut self, texture: impl Into<TextureRef<'a>>, params: impl Into<Option<SpriteDrawParams>>, transform: impl Into<Option<Transform>>) {
        let texture = texture.into();
        let params = params.into().unwrap_or_default();
        let transform = transform.into().unwrap_or_default();

        self.switch_draw_command(DrawCommand {
            texture: texture.texture().unwrap_or_else(|| self.default_texture.clone()),
            primitive: PrimitiveType::Triangles,
        });

        let texture_size = {
            let texture_size = texture.texture_size();
            Size::new(texture_size.width as f32, texture_size.height as f32)
        };
        let region = params.region.unwrap_or_else(|| Region::new(0.0, 0.0, texture_size.width, texture_size.height));
        let origin = params.origin.unwrap_or_else(|| Position::zero());
        let matrix = self.transform_matrix * transform.0;
        let x0y0 = matrix * Vec4::new(-origin.x, -origin.y, 0.0, 1.0);
        let x1y0 = matrix * Vec4::new(-origin.x + region.width, -origin.y, 0.0, 1.0);
        let x0y1 = matrix * Vec4::new(-origin.x, -origin.y + region.height, 0.0, 1.0);
        let x1y1 = matrix * Vec4::new(-origin.x + region.width, -origin.y + region.height, 0.0, 1.0);
        let uv = Region::new(
            region.x / texture_size.width,
            region.y / texture_size.height,
            region.width / texture_size.width,
            region.height / texture_size.height,
        );
        let colors = params.colors.unwrap_or_else(|| [Color::WHITE, Color::WHITE, Color::WHITE, Color::WHITE]);

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

    pub fn draw_text(&mut self, font: &Font, text: &str, params: impl Into<Option<TextDrawParams>>, transform: impl Into<Option<Transform>>) {
        let params = params.into().unwrap_or_default();
        let transform = transform.into().unwrap_or_default();

        self.switch_draw_command(DrawCommand {
            texture: font.cache_texture(),
            primitive: PrimitiveType::Triangles,
        });

        let text_size = params.text_size.unwrap_or(14.0);
        let line_metrics = font.line_metrics(text_size);
        let char_spacing = params.char_spacing.unwrap_or(0.0);
        let line_height = params.line_height.unwrap_or(line_metrics.height);
        let line_spacing = params.line_spacing.unwrap_or(line_metrics.line_gap);
        let wrap_width = params.wrap_width.unwrap_or(0.0).max(0.0);
        let wrap_height = params.wrap_height.unwrap_or(0.0).max(0.0);
        let horizontal_gravity = params.horizontal_gravity.unwrap_or(TextLayoutGravity::default());
        let vertical_gravity = params.vertical_gravity.unwrap_or(TextLayoutGravity::default());
        let origin = params.origin.unwrap_or_else(|| Position::zero());
        let matrix = self.transform_matrix * transform.0;
        let color = params.color.unwrap_or(Color::WHITE);
        let graphics_scale_factor = {
            if font.is_fit_hidpi() && self.canvas.is_none() {
                self.window().scale_factor() as f32
            } else {
                1.0
            }
        };

        let (line_layout_infos, layout_height) = {
            let mut line_layout_infos = Vec::new();
            let mut glyph_positions = Vec::new();
            let mut caret = Position::zero();
            for c in text.chars() {
                if c.is_control() {
                    match c {
                        '\n' => {
                            line_layout_infos.push((glyph_positions, caret.x));
                            glyph_positions = Vec::new();
                            caret.x = 0.0;
                            if caret.y > 0.0 {
                                caret.y += line_spacing;
                            }
                            caret.y += line_height;
                        }
                        _ => (),
                    }
                } else {
                    let glyph_id = font.glyph_id(c);
                    let glyph_metrics = font.glyph_metrics(glyph_id, text_size);
                    if wrap_width > 0.0 && caret.x > 0.0 && caret.x + glyph_metrics.advance_width > wrap_width {
                        line_layout_infos.push((glyph_positions, caret.x));
                        glyph_positions = Vec::new();
                        caret.x = 0.0;
                        if caret.y > 0.0 {
                            caret.y += line_spacing;
                        }
                        caret.y += line_height;
                    }
                    glyph_positions.push((c, caret));
                    if caret.x > 0.0 {
                        caret.x += char_spacing;
                    }
                    caret.x += glyph_metrics.advance_width;
                }
            }
            if !glyph_positions.is_empty() {
                line_layout_infos.push((glyph_positions, caret.x));
                if caret.y > 0.0 {
                    caret.y += line_spacing;
                }
                caret.y += line_height;
            }
            (line_layout_infos, caret.y)
        };

        let offset_y = match vertical_gravity {
            TextLayoutGravity::Start => 0.0,
            TextLayoutGravity::Center => (wrap_height - layout_height) / 2.0,
            TextLayoutGravity::End => wrap_height - layout_height,
        } - origin.y;
        for (glyph_positions, layout_width) in line_layout_infos {
            let offset_x = match horizontal_gravity {
                TextLayoutGravity::Start => 0.0,
                TextLayoutGravity::Center => (wrap_width - layout_width) / 2.0,
                TextLayoutGravity::End => wrap_width - layout_width,
            } - origin.x;
            for (c, glyph_position) in glyph_positions {
                loop {
                    match font.cache_glyph(c, text_size, graphics_scale_factor) {
                        Ok(cached_by) => {
                            let draw_info = match cached_by {
                                font::CachedBy::Added(draw_info) => draw_info,
                                font::CachedBy::Existed(draw_info) => draw_info,
                            };
                            if let Some(draw_info) = draw_info {
                                let glyph_position = Position::new(
                                    glyph_position.x + draw_info.bounds.min_x(),
                                    glyph_position.y + line_metrics.ascent + draw_info.bounds.min_y() + (line_height - line_metrics.height) / 2.0,
                                );
                                let x0y0 = matrix * Vec4::new(offset_x + glyph_position.x, offset_y + glyph_position.y, 0.0, 1.0);
                                let x1y0 = matrix * Vec4::new(offset_x + glyph_position.x + draw_info.bounds.width, offset_y + glyph_position.y, 0.0, 1.0);
                                let x0y1 = matrix * Vec4::new(offset_x + glyph_position.x, offset_y + glyph_position.y + draw_info.bounds.height, 0.0, 1.0);
                                let x1y1 = matrix * Vec4::new(offset_x + glyph_position.x + draw_info.bounds.width, offset_y + glyph_position.y + draw_info.bounds.height, 0.0, 1.0);

                                let vertices = vec![
                                    Vertex {
                                        position: Position::new(x0y0.x(), x0y0.y()),
                                        uv: draw_info.uv.top_left(),
                                        color,
                                    },
                                    Vertex {
                                        position: Position::new(x1y0.x(), x1y0.y()),
                                        uv: draw_info.uv.top_right(),
                                        color,
                                    },
                                    Vertex {
                                        position: Position::new(x0y1.x(), x0y1.y()),
                                        uv: draw_info.uv.bottom_left(),
                                        color,
                                    },
                                    Vertex {
                                        position: Position::new(x1y1.x(), x1y1.y()),
                                        uv: draw_info.uv.bottom_right(),
                                        color,
                                    },
                                ];
                                let elements = SPRITE_ELEMENTS.to_vec();
                                self.append_vertices_and_elements(vertices, Some(elements));
                            }
                            break;
                        }
                        Err(cache_error) => {
                            let cache_size_maximized = font.cache_texture_size() >= self.max_texture_size;
                            match (cache_error, cache_size_maximized) {
                                (font::CacheError::TooLarge, true) => panic!("char is too large"),
                                (font::CacheError::NoRoom, true) => {
                                    self.flush();
                                    font.clear_cache();
                                }
                                _ => {
                                    self.flush();
                                    font.resize_cache((font.cache_texture_size() * 2).min(self.max_texture_size));
                                }
                            }
                        }
                    }
                }
            }
        }
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
            renderer_vertex_size: SPRITE_VERTEX_COUNT * 2048,
            renderer_element_size: SPRITE_ELEMENT_COUNT * 2048,
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
