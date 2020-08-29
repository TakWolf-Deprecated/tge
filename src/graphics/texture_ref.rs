use super::{opengl, Texture, Canvas, Font};
use crate::math::Size;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum TextureRef<'a> {
    Texture(&'a Texture),
    Canvas(&'a Canvas),
    Font(&'a Font),
    None,
}

impl TextureRef<'_> {
    pub(crate) fn texture(&self) -> Option<Rc<opengl::Texture>> {
        match self {
            Self::Texture(texture) => Some(texture.texture().clone()),
            Self::Canvas(canvas) => Some(canvas.texture().clone()),
            Self::Font(font) => Some(font.cache_texture()),
            Self::None => None,
        }
    }

    pub fn texture_size(&self) -> Option<Size<u32>> {
        match self {
            Self::Texture(texture) => Some(texture.size()),
            Self::Canvas(canvas) => Some(canvas.size()),
            Self::Font(font) => {
                let cache_texture_size = font.cache_texture_size();
                Some(Size::new(cache_texture_size, cache_texture_size))
            }
            Self::None => None,
        }
    }
}

impl<'a> From<&'a Texture> for TextureRef<'a> {
    fn from(texture: &'a Texture) -> Self {
        Self::Texture(texture)
    }
}

impl<'a> From<&'a Canvas> for TextureRef<'a> {
    fn from(canvas: &'a Canvas) -> Self {
        Self::Canvas(canvas)
    }
}

impl<'a> From<&'a Font> for TextureRef<'a> {
    fn from(font: &'a Font) -> Self {
        Self::Font(font)
    }
}
