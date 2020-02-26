use super::opengl::{VertexArray, BufferUsage, Buffer, VertexBuffer, ElementBuffer, PrimitiveType};
use super::vertex;
use super::vertex::{Vertex, Vertices};
use crate::error::{GameError, GameResult};
use crate::engine::Engine;
use glow::Context;
use std::rc::Rc;

pub struct Mesh {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    vertex_buffer_size: usize,
    element_buffer: Option<ElementBuffer>,
    element_buffer_size: Option<usize>,
}

impl Mesh {

    pub fn builder(engine: &mut Engine) -> GameResult<MeshBuilder> {
        MeshBuilder::new(engine.graphics().gl().clone())
    }

    pub(crate) fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }

    pub fn init_vertex_buffer_size(&mut self, usage: BufferUsage, size: usize) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_size(usage, size);
        self.vertex_buffer.unbind();
        self.vertex_buffer_size = size;
    }

    pub fn init_vertex_buffer_with_data(&mut self, usage: BufferUsage, data: &[f32]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_with_data(usage, data);
        self.vertex_buffer.unbind();
        self.vertex_buffer_size = data.len();
    }

    pub fn init_vertex_buffer_with_vertices(&mut self, usage: BufferUsage, vertices: &[Vertex]) {
        self.init_vertex_buffer_with_data(usage, &vertices.to_raw_data());
    }

    pub fn update_vertex_data(&self, offset: usize, data: &[f32]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.sub_data(offset, data);
        self.vertex_buffer.unbind();
    }

    pub fn update_vertices(&self, offset: usize, vertices: &[Vertex]) {
        self.update_vertex_data(offset, &vertices.to_raw_data());
    }

    pub fn vertex_buffer_size(&self) -> usize {
        self.vertex_buffer_size
    }

    fn element_buffer(&self) -> GameResult<&ElementBuffer> {
        self.element_buffer.as_ref().ok_or_else(|| GameError::StateError("not setup element buffer".into()))
    }

    pub fn init_element_buffer_size(&mut self, usage: BufferUsage, size: usize) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.init_size(usage, size);
        element_buffer.unbind();
        self.element_buffer_size = Some(size);
        Ok(())
    }

    pub fn init_element_buffer_with_data(&mut self, usage: BufferUsage, data: &[u32]) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.init_with_data(usage, data);
        element_buffer.unbind();
        self.element_buffer_size = Some(data.len());
        Ok(())
    }

    pub fn update_element_data(&self, offset: usize, data: &[u32]) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.sub_data(offset, data);
        element_buffer.unbind();
        Ok(())
    }

    pub fn element_buffer_size(&self) -> Option<usize> {
        self.element_buffer_size
    }

    pub(crate) fn draw_arrays(&self, primitive: PrimitiveType, first: usize, count: usize) {
        self.vertex_array.bind();
        self.vertex_array.draw_arrays(primitive, first, count);
        self.vertex_array.unbind();
    }

    pub(crate) fn draw_elements(&self, primitive: PrimitiveType, count: usize, offset: usize) {
        self.vertex_array.bind();
        self.vertex_array.draw_elements(primitive, count, offset);
        self.vertex_array.unbind();
    }

}

pub struct MeshBuilder {
    gl: Rc<Context>,
    vertex_array: VertexArray,
    vertex_buffer: Option<VertexBuffer>,
    vertex_buffer_size: Option<usize>,
    element_buffer: Option<ElementBuffer>,
    element_buffer_size: Option<usize>,
}

impl MeshBuilder {

    pub(crate) fn new(gl: Rc<Context>) -> GameResult<Self> {
        let vertex_array = VertexArray::new(gl.clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        vertex_array.bind();
        Ok(Self {
            gl,
            vertex_array,
            vertex_buffer: None,
            vertex_buffer_size: None,
            element_buffer: None,
            element_buffer_size: None,
        })
    }

    fn assert_vertex_buffer_not_init(&self) {
        assert!(self.vertex_buffer.is_none(), "vertex buffer has been setup");
    }

    pub fn init_vertex_buffer_size(mut self, usage: BufferUsage, size: usize) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_size(usage, size);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_buffer_size = Some(size);
        self
    }

    pub fn init_vertex_buffer_with_data(mut self, usage: BufferUsage, data: &[f32]) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_with_data(usage, data);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_buffer_size = Some(data.len());
        self
    }

    pub fn init_vertex_buffer_with_vertices(mut self, usage: BufferUsage, vertices: &[Vertex]) -> Self {
        self.init_vertex_buffer_with_data(usage, &vertices.to_raw_data())
    }

    fn vertex_buffer(&self) -> &VertexBuffer {
        self.vertex_buffer.as_ref().expect("vertex buffer has not been setup")
    }

    pub fn vertex_attribute_pointer(self, index: usize, size: usize, stride: usize, offset: usize) -> Self {
        self.vertex_buffer().set_attrib_pointer_f32(index, size, stride, offset);
        self
    }

    pub fn default_vertex_attribute_pointer(self) -> Self {
        let vertex_buffer = self.vertex_buffer();
        vertex_buffer.set_attrib_pointer_f32(0, vertex::ATTRIBUTE_POSITION_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_0);
        vertex_buffer.set_attrib_pointer_f32(1, vertex::ATTRIBUTE_UV_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_1);
        vertex_buffer.set_attrib_pointer_f32(2, vertex::ATTRIBUTE_COLOR_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_2);
        self
    }

    fn assert_element_buffer_not_init(&self) {
        assert!(self.element_buffer.is_none(), "element buffer has been setup");
    }

    pub fn init_element_buffer_size(mut self, usage: BufferUsage, size: usize) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_size(usage, size);
        self.element_buffer = Some(element_buffer);
        self.element_buffer_size = Some(size);
        self
    }

    pub fn init_element_buffer_with_data(mut self, usage: BufferUsage, data: &[u32]) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_with_data(usage, data);
        self.element_buffer = Some(element_buffer);
        self.element_buffer_size = Some(data.len());
        self
    }

    pub fn build(self) -> GameResult<Mesh> {
        let vertex_array = self.vertex_array;
        let vertex_buffer = self.vertex_buffer
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let vertex_buffer_size = self.vertex_buffer_size
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let element_buffer = self.element_buffer;
        let element_buffer_size = self.element_buffer_size;
        vertex_array.unbind();
        vertex_buffer.unbind();
        if let Some(element_buffer) = element_buffer.as_ref() {
            element_buffer.unbind();
        }
        Ok(Mesh {
            vertex_array,
            vertex_buffer,
            vertex_buffer_size,
            element_buffer,
            element_buffer_size,
        })
    }

}
