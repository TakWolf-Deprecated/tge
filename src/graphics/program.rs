use super::opengl;
use crate::error::{GameError, GameResult};
use crate::engine::Engine;
use glow::Context;
use std::rc::Rc;
use std::path::Path;

const DEFAULT_VERTEX_SHADER_SOURCE: &str = include_str!("shaders/default.vert");
const DEFAULT_FRAGMENT_SHADER_SOURCE: &str = include_str!("shaders/default.frag");

pub struct Program {
    program: Rc<opengl::Program>,
}

impl Program {

    pub fn new(
        engine: &mut Engine,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> GameResult<Self> {
        let program = opengl::Program::new(
            engine.graphics().gl().clone(),
            vertex_shader_source,
            fragment_shader_source,
        ).map_err(|error| GameError::InitError(error.into()))?;
        Ok(Self { program: Rc::new(program) })
    }

    pub fn load<P: AsRef<Path>>(
        engine: &mut Engine,
        vertex_shader_path: P,
        fragment_shader_path: P,
    ) -> GameResult<Self> {
        let vertex_shader_source = std::fs::read_to_string(vertex_shader_path)
            .map_err(|error| GameError::IoError(Box::new(error)))?;
        let fragment_shader_source = std::fs::read_to_string(fragment_shader_path)
            .map_err(|error| GameError::IoError(Box::new(error)))?;
        Self::new(engine, &vertex_shader_source, &fragment_shader_source)
    }

    pub(crate) fn default(gl: Rc<Context>) -> GameResult<Rc<opengl::Program>> {
        let program = super::opengl::Program::new(
            gl,
            DEFAULT_VERTEX_SHADER_SOURCE,
            DEFAULT_FRAGMENT_SHADER_SOURCE,
        ).map_err(|error| GameError::InitError(error.into()))?;
        Ok(Rc::new(program))
    }

    pub(crate) fn program(&self) -> &Rc<opengl::Program> {
        &self.program
    }

}
