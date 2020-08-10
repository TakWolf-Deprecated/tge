use super::{opengl, Texture, Canvas, Font};
use crate::math::Size;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum TextureHolder<'a> {
    Texture(&'a Texture),
    Canvas(&'a Canvas),
    Font(&'a Font),
    None,
}

impl TextureHolder<'_> {

    pub(crate) fn clone_texture(&self) -> Option<Rc<opengl::Texture>> {
        match self {
            Self::Texture(texture) => Some(texture.texture().clone()),
            Self::Canvas(canvas) => Some(canvas.texture().clone()),
            Self::Font(font) => Some(font.clone_cache_texture()),
            Self::None => None,
        }
    }

    pub(crate) fn texture_size(&self) -> Size<u32> {
        match self {
            Self::Texture(texture) => texture.size(),
            Self::Canvas(canvas) => canvas.size(),
            Self::Font(font) => {
                let cache_texture_size = font.cache_texture_size();
                Size::new(cache_texture_size, cache_texture_size)
            }
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

impl<'a> From<&'a Font> for TextureHolder<'a> {

    fn from(font: &'a Font) -> Self {
        Self::Font(font)
    }

}
