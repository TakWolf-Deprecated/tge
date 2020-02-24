use super::PrimitiveType;
use glow::{Context, HasContext};
use std::rc::Rc;

pub type VertexArrayId = <Context as HasContext>::VertexArray;

pub struct VertexArray {
    gl: Rc<Context>,
    id: VertexArrayId,
}

impl VertexArray {

    pub fn new(gl: Rc<Context>) -> Result<Self, String> {
        let id = unsafe {
            gl.create_vertex_array()?
        };
        Ok(Self { gl, id })
    }

    pub fn id(&self) -> VertexArrayId {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn draw_arrays(&self, primitive: PrimitiveType, first: usize, count: usize) {
        unsafe {
            self.gl.draw_arrays(primitive.to_flag(), first as i32, count as i32);
        }
    }

    pub fn draw_elements(&self, primitive: PrimitiveType, count: usize, offset: usize) {
        unsafe {
            self.gl.draw_elements(primitive.to_flag(), count as i32, glow::UNSIGNED_INT, offset as i32);
        }
    }

}

impl Drop for VertexArray {

    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.id);
        }
    }

}

impl PartialEq for VertexArray {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}
