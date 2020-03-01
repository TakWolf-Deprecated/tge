use super::{vertex, Vertex};
use super::opengl::{VertexArray, BufferUsage, Buffer, VertexBuffer, ElementBuffer, PrimitiveType};
use crate::error::{GameError, GameResult};
use glow::Context;
use std::rc::Rc;

pub struct Renderer {
    vertex_array: VertexArray,
    vertex_buffer: VertexBuffer,
    vertex_size: usize,
    element_buffer: Option<ElementBuffer>,
    element_size: Option<usize>,
}

impl Renderer {

    pub fn init_vertex_size(&mut self, usage: BufferUsage, size: usize) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_size(usage, vertex::ATTRIBUTE_STRIDE * size);
        init_vertex_attribute_pointer(&self.vertex_buffer);
        self.vertex_buffer.unbind();
        self.vertex_size = size;
    }

    pub fn init_with_vertices(&mut self, usage: BufferUsage, vertices: &[Vertex]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.init_with_data(usage, &convert_vertices_to_data(vertices));
        init_vertex_attribute_pointer(&self.vertex_buffer);
        self.vertex_buffer.unbind();
        self.vertex_size = vertices.len();
    }

    pub fn update_vertices(&self, offset: usize, vertices: &[Vertex]) {
        self.vertex_buffer.bind();
        self.vertex_buffer.sub_data(offset, &convert_vertices_to_data(vertices));
        self.vertex_buffer.unbind();
    }

    pub fn vertex_size(&self) -> usize {
        self.vertex_size
    }

    fn element_buffer(&self) -> GameResult<&ElementBuffer> {
        self.element_buffer.as_ref().ok_or_else(|| GameError::StateError("not setup element buffer".into()))
    }

    pub fn init_element_size(&mut self, usage: BufferUsage, size: usize) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.init_size(usage, size);
        element_buffer.unbind();
        self.element_size = Some(size);
        Ok(())
    }

    pub fn init_with_elements(&mut self, usage: BufferUsage, elements: &[u32]) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.init_with_data(usage, elements);
        element_buffer.unbind();
        self.element_size = Some(elements.len());
        Ok(())
    }

    pub fn update_elements(&self, offset: usize, elements: &[u32]) -> GameResult {
        let element_buffer = self.element_buffer()?;
        element_buffer.bind();
        element_buffer.sub_data(offset, elements);
        element_buffer.unbind();
        Ok(())
    }

    pub fn element_size(&self) -> Option<usize> {
        self.element_size
    }

    pub fn draw_arrays(&self, primitive: PrimitiveType, first: usize, count: usize) {
        self.vertex_array.bind();
        self.vertex_array.draw_arrays(primitive, first, count);
        self.vertex_array.unbind();
    }

    pub fn draw_elements(&self, primitive: PrimitiveType, count: usize, offset: usize) {
        self.vertex_array.bind();
        self.vertex_array.draw_elements(primitive, count, offset);
        self.vertex_array.unbind();
    }

}

pub struct RendererBuilder {
    gl: Rc<Context>,
    vertex_array: VertexArray,
    vertex_buffer: Option<VertexBuffer>,
    vertex_size: Option<usize>,
    element_buffer: Option<ElementBuffer>,
    element_size: Option<usize>,
}

impl RendererBuilder {

    pub fn new(gl: Rc<Context>) -> GameResult<Self> {
        let vertex_array = VertexArray::new(gl.clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        vertex_array.bind();
        Ok(Self {
            gl,
            vertex_array,
            vertex_buffer: None,
            vertex_size: None,
            element_buffer: None,
            element_size: None,
        })
    }

    fn assert_vertex_buffer_not_init(&self) {
        assert!(self.vertex_buffer.is_none(), "vertex buffer has been setup");
    }

    pub fn init_vertex_size(mut self, usage: BufferUsage, size: usize) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_size(usage, vertex::ATTRIBUTE_STRIDE * size);
        init_vertex_attribute_pointer(&vertex_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_size = Some(size);
        self
    }

    pub fn init_with_vertices(mut self, usage: BufferUsage, vertices: &[Vertex]) -> Self {
        self.assert_vertex_buffer_not_init();
        let vertex_buffer = Buffer::new_vertex(self.gl.clone()).unwrap();
        vertex_buffer.bind();
        vertex_buffer.init_with_data(usage, &convert_vertices_to_data(vertices));
        init_vertex_attribute_pointer(&vertex_buffer);
        self.vertex_buffer = Some(vertex_buffer);
        self.vertex_size = Some(vertices.len());
        self
    }

    fn assert_element_buffer_not_init(&self) {
        assert!(self.element_buffer.is_none(), "element buffer has been setup");
    }

    pub fn init_element_size(mut self, usage: BufferUsage, size: usize) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_size(usage, size);
        self.element_buffer = Some(element_buffer);
        self.element_size = Some(size);
        self
    }

    pub fn init_with_elements(mut self, usage: BufferUsage, elements: &[u32]) -> Self {
        self.assert_element_buffer_not_init();
        let element_buffer = Buffer::new_element(self.gl.clone()).unwrap();
        element_buffer.bind();
        element_buffer.init_with_data(usage, elements);
        self.element_buffer = Some(element_buffer);
        self.element_size = Some(elements.len());
        self
    }

    pub fn build(self) -> GameResult<Renderer> {
        let vertex_array = self.vertex_array;
        let vertex_buffer = self.vertex_buffer
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let vertex_size = self.vertex_size
            .ok_or_else(|| GameError::InitError("must setup vertex buffer".into()))?;
        let element_buffer = self.element_buffer;
        let element_size = self.element_size;
        vertex_array.unbind();
        vertex_buffer.unbind();
        if let Some(element_buffer) = element_buffer.as_ref() {
            element_buffer.unbind();
        }
        Ok(Renderer {
            vertex_array,
            vertex_buffer,
            vertex_size,
            element_buffer,
            element_size,
        })
    }

}

fn init_vertex_attribute_pointer(vertex_buffer: &VertexBuffer) {
    vertex_buffer.set_attrib_pointer_f32(0, vertex::ATTRIBUTE_POSITION_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_0);
    vertex_buffer.set_attrib_pointer_f32(1, vertex::ATTRIBUTE_UV_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_1);
    vertex_buffer.set_attrib_pointer_f32(2, vertex::ATTRIBUTE_COLOR_SIZE, vertex::ATTRIBUTE_STRIDE, vertex::ATTRIBUTE_OFFSET_2);
}

fn convert_vertices_to_data(vertices: &[Vertex]) -> Vec<f32> {
    let mut data = Vec::with_capacity(vertex::ATTRIBUTE_STRIDE * vertices.len());
    for vertex in vertices {
        data.push(vertex.position.x);
        data.push(vertex.position.y);
        data.push(vertex.uv.x);
        data.push(vertex.uv.y);
        data.push(vertex.color.red);
        data.push(vertex.color.green);
        data.push(vertex.color.blue);
        data.push(vertex.color.alpha);
    }
    data
}
