use super::{opengl, Filter, Wrap, Texture};
use super::opengl::{Attachment, Framebuffer};
use crate::error::{GameError, GameResult};
use crate::math::Size;
use crate::engine::Engine;
use std::rc::Rc;

pub struct Canvas {
    framebuffer: Rc<Framebuffer>,
    texture: Texture,
}

impl Canvas {
    pub fn new(engine: &mut Engine, size: impl Into<Size<u32>>) -> GameResult<Self> {
        let framebuffer = Framebuffer::new(engine.graphics().gl())
            .map_err(|error| GameError::InitError(error.into()))?;
        let texture = Texture::new(engine, size, None)?;
        framebuffer.bind();
        framebuffer.attach_texture(Attachment::Color(0), Some(texture.texture().id()));
        framebuffer.check_status().map_err(|error| GameError::InitError(error.into()))?;
        framebuffer.unbind();
        Ok(Self {
            framebuffer: Rc::new(framebuffer),
            texture,
        })
    }

    pub(crate) fn framebuffer(&self) -> &Rc<Framebuffer> {
        &self.framebuffer
    }

    pub(crate) fn texture(&self) -> &Rc<opengl::Texture> {
        self.texture.texture()
    }

    pub fn size(&self) -> Size<u32> {
        self.texture.size()
    }

    pub fn filter(&self) -> Filter {
        self.texture.filter()
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.texture.set_filter(filter)
    }

    pub fn wrap(&self) -> Wrap {
        self.texture.wrap()
    }

    pub fn set_wrap(&mut self, wrap: Wrap) {
        self.texture.set_wrap(wrap)
    }
}
