use glow::{Context, HasContext};
use std::rc::Rc;

pub type ProgramId = <Context as HasContext>::Program;

pub struct Program {
    gl: Rc<Context>,
    id: ProgramId,
}

impl Program {

    pub fn new(gl: Rc<Context>, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Self, String> {
        let id = unsafe {
            let vertex_shader_id = gl.create_shader(glow::VERTEX_SHADER)?;
            gl.shader_source(vertex_shader_id, vertex_shader_source);
            gl.compile_shader(vertex_shader_id);
            if !gl.get_shader_compile_status(vertex_shader_id) {
                return Err(gl.get_shader_info_log(vertex_shader_id));
            }

            let fragment_shader_id = gl.create_shader(glow::FRAGMENT_SHADER)?;
            gl.shader_source(fragment_shader_id, fragment_shader_source);
            gl.compile_shader(fragment_shader_id);
            if !gl.get_shader_compile_status(fragment_shader_id) {
                return Err(gl.get_shader_info_log(fragment_shader_id));
            }

            let program_id = gl.create_program()?;

            gl.attach_shader(program_id, vertex_shader_id);
            gl.attach_shader(program_id, fragment_shader_id);

            gl.link_program(program_id);
            if !gl.get_program_link_status(program_id) {
                return Err(gl.get_program_info_log(program_id));
            }

            gl.delete_shader(vertex_shader_id);
            gl.delete_shader(fragment_shader_id);

            program_id
        };
        Ok(Self { gl, id })
    }

    pub fn id(&self) -> ProgramId {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.use_program(None);
        }
    }

    pub fn set_uniform_matrix_4(&self, name: &str, mat4: &[f32; 16]) {
        unsafe {
            let location = self.gl.get_uniform_location(self.id, name);
            self.gl.uniform_matrix_4_f32_slice(location.as_ref(), false, mat4);
        }
    }

}

impl Drop for Program {

    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.id);
        }
    }

}

impl PartialEq for Program {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}
