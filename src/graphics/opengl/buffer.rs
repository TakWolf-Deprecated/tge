use glow::{Context, HasContext};
use std::rc::Rc;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BufferTarget {
    Vertex,
    Element,
}

impl BufferTarget {

    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            BufferTarget::Vertex => glow::ARRAY_BUFFER,
            BufferTarget::Element => glow::ELEMENT_ARRAY_BUFFER,
        }
    }

}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

impl BufferUsage {

    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            BufferUsage::Static => glow::STATIC_DRAW,
            BufferUsage::Dynamic => glow::DYNAMIC_DRAW,
            BufferUsage::Stream => glow::STREAM_DRAW,
        }
    }

}

pub type BufferId = <Context as HasContext>::Buffer;

pub struct Buffer<T> {
    gl: Rc<Context>,
    id: BufferId,
    target: BufferTarget,
    phantom: PhantomData<T>,
    unit_bytes_size: usize,
}

impl<T> Buffer<T> {

    pub fn new(gl: Rc<Context>, target: BufferTarget) -> Result<Self, String> {
        let id = unsafe {
            gl.create_buffer()?
        };
        let unit_bytes_size = std::mem::size_of::<T>();
        Ok(Self {
            gl,
            id,
            target,
            phantom: PhantomData,
            unit_bytes_size,
        })
    }

    pub fn id(&self) -> BufferId {
        self.id
    }

    pub fn target(&self) -> BufferTarget {
        self.target
    }

    pub fn unit_bytes_size(&self) -> usize {
        self.unit_bytes_size
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_buffer(self.target.to_flag(), Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_buffer(self.target.to_flag(), None);
        }
    }

    pub fn init_data(&self, usage: BufferUsage, data: &[T]) {
        unsafe {
            self.gl.buffer_data_u8_slice(
                self.target.to_flag(),
                std::slice::from_raw_parts(data.as_ptr().cast(), std::mem::size_of_val(data)),
                usage.to_flag(),
            );
        }
    }

    pub fn init_data_size(&self, usage: BufferUsage, size: usize) {
        unsafe {
            self.gl.buffer_data_size(
                self.target.to_flag(),
                (self.unit_bytes_size * size) as i32,
                usage.to_flag(),
            );
        }
    }

    pub fn sub_data(&self, offset: usize, data: &[T]) {
        unsafe {
            self.gl.buffer_sub_data_u8_slice(
                self.target.to_flag(),
                (self.unit_bytes_size * offset) as i32,
                std::slice::from_raw_parts(data.as_ptr().cast(), std::mem::size_of_val(data)),
            );
        }
    }

}

impl<T> Drop for Buffer<T> {

    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.id);
        }
    }

}

impl<T> PartialEq for Buffer<T> {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

const F32_BYTES_SIZE: usize = std::mem::size_of::<f32>();

pub type VertexBuffer = Buffer<f32>;

impl VertexBuffer {

    pub fn new_vertex(gl: Rc<Context>) -> Result<Self, String> {
        Self::new(gl, BufferTarget::Vertex)
    }

    pub fn set_attrib_pointer_f32(&self, index: usize, size: usize, stride: usize, offset: usize) {
        unsafe {
            self.gl.vertex_attrib_pointer_f32(
                index as u32,
                size as i32,
                glow::FLOAT,
                false,
                (F32_BYTES_SIZE * stride) as i32,
                (F32_BYTES_SIZE * offset) as i32,
            );
            self.gl.enable_vertex_attrib_array(index as u32);
        }
    }

}

pub type ElementBuffer = Buffer<u32>;

impl ElementBuffer {

    pub fn new_element(gl: Rc<Context>) -> Result<Self, String> {
        Self::new(gl, BufferTarget::Element)
    }

}
