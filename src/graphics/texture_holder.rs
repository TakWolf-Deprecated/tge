use super::{opengl, Texture, Canvas};
use crate::math::Size;
use std::rc::Rc;

#[derive(Clone)]
pub enum TextureHolder<'a> {
    Texture(&'a Texture),
    Canvas(&'a Canvas),
    None,
}

impl TextureHolder<'_> {

    pub(crate) fn texture(&self) -> Option<&Rc<opengl::Texture>> {
        match self {
            Self::Texture(texture) => Some(texture.texture()),
            Self::Canvas(canvas) => Some(canvas.texture()),
            Self::None => None,
        }
    }

    pub(crate) fn texture_size(&self) -> Size<u32> {
        match self {
            Self::Texture(texture) => texture.size(),
            Self::Canvas(canvas) => canvas.size(),
            Self::None => Size::zero(),
        }
    }

}

impl<'a> From<&'a Texture> for TextureHolder<'a> {

    fn from(texture: &'a Texture) -> Self {
        Self::Texture(texture)
    }

}

impl<'a> From<&'a Canvas> for TextureHolder<'a> {

    fn from(canvas: &'a Canvas) -> Self {
        Self::Canvas(canvas)
    }

}

pub const NO_TEXTURE: TextureHolder = TextureHolder::None;
