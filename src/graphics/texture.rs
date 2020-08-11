use super::{opengl, FilterMode, Filter, WrapMode, Wrap, Image, validate_pixels};
use crate::error::{GameError, GameResult};
use crate::math::{Size, Region};
use crate::engine::Engine;
use glow::Context;
use std::rc::Rc;
use std::path::Path;

pub struct Texture {
    texture: Rc<opengl::Texture>,
    size: Size<u32>,
    filter: Filter,
    mipmap_generated: bool,
    wrap: Wrap,
}

impl Texture {
    pub fn new(engine: &mut Engine, size: impl Into<Size<u32>>, pixels: Option<&[u8]>) -> GameResult<Self> {
        let size = size.into();
        if let Some(pixels) = pixels {
            validate_pixels(size, pixels)?;
        }
        let filter = engine.graphics().default_filter();
        let generate_mipmap = filter.mipmap.is_some();
        let wrap = engine.graphics().default_wrap();
        let texture = opengl::Texture::new(engine.graphics().gl().clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image(size.width, size.height, pixels);
        texture.set_filter(filter);
        if generate_mipmap {
            texture.generate_mipmap();
        }
        texture.set_wrap(wrap);
        texture.unbind();
        Ok(Self {
            texture: Rc::new(texture),
            size,
            filter,
            mipmap_generated: generate_mipmap,
            wrap,
        })
    }

    pub fn from_image(engine: &mut Engine, image: &Image) -> GameResult<Self> {
        let size = image.size();
        let pixels = image.pixels();
        Self::new(engine, size, Some(pixels))
    }

    pub fn from_bytes(engine: &mut Engine, bytes: &[u8]) -> GameResult<Self> {
        let image = Image::from_bytes(bytes)?;
        Self::from_image(engine, &image)
    }

    pub fn load(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<Self> {
        let image = Image::load(engine, path)?;
        Self::from_image(engine, &image)
    }

    pub(crate) fn default(gl: Rc<Context>) -> GameResult<Rc<opengl::Texture>> {
        let texture = opengl::Texture::new(gl)
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image(1, 1, Some(&[255, 255, 255, 255]));
        texture.set_filter(Filter::new(FilterMode::Nearest, FilterMode::Nearest, None));
        texture.set_wrap(Wrap::uv(WrapMode::Repeat, WrapMode::Repeat));
        texture.unbind();
        Ok(Rc::new(texture))
    }

    pub(crate) fn for_font_cache(engine: &mut Engine, size: u32) -> GameResult<Self> {
        let size = Size::new(size, size);
        let filter = engine.graphics().default_filter();
        let generate_mipmap = filter.mipmap.is_some();
        let wrap = Wrap::uv(WrapMode::Repeat, WrapMode::Repeat);
        let texture = opengl::Texture::new(engine.graphics().gl().clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image(size.width, size.height, None);
        texture.set_filter(filter);
        if generate_mipmap {
            texture.generate_mipmap();
        }
        texture.set_wrap(wrap);
        texture.unbind();
        Ok(Self {
            texture: Rc::new(texture),
            size,
            filter,
            mipmap_generated: generate_mipmap,
            wrap,
        })
    }

    pub(crate) fn texture(&self) -> &Rc<opengl::Texture> {
        &self.texture
    }

    pub fn size(&self) -> Size<u32> {
        self.size
    }

    pub fn filter(&self) -> Filter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: Filter) {
        if self.filter != filter {
            self.texture.bind();
            self.texture.set_filter(filter);
            if !self.mipmap_generated && filter.mipmap.is_some() {
                self.texture.generate_mipmap();
                self.mipmap_generated = true;
            }
            self.texture.unbind();
            self.filter = filter;
        }
    }

    pub fn wrap(&self) -> Wrap {
        self.wrap
    }

    pub fn set_wrap(&mut self, wrap: Wrap) {
        if self.wrap != wrap {
            self.texture.bind();
            self.texture.set_wrap(wrap);
            self.texture.unbind();
            self.wrap = wrap;
        }
    }

    pub fn init_pixels(&mut self, size: impl Into<Size<u32>>, pixels: Option<&[u8]>) -> GameResult {
        let size = size.into();
        if let Some(pixels) = pixels {
            validate_pixels(size, pixels)?;
        }
        self.texture.bind();
        self.texture.init_image(size.width, size.height, pixels);
        self.size = size;
        if self.filter.mipmap.is_some() {
            self.texture.generate_mipmap();
            self.mipmap_generated = true;
        } else {
            self.mipmap_generated = false;
        }
        self.texture.unbind();
        Ok(())
    }

    pub fn init_with_image(&mut self, image: &Image) -> GameResult {
        let size = image.size();
        let pixels = image.pixels();
        self.init_pixels(size, Some(pixels))
    }

    pub fn update_pixels(&mut self, region: impl Into<Region<u32>>, pixels: Option<&[u8]>) -> GameResult {
        let region = region.into();
        if let Some(pixels) = pixels {
            validate_pixels(region.size(), pixels)?;
        }
        self.texture.bind();
        self.texture.sub_image(
            region.x,
            region.y,
            region.width,
            region.height,
            pixels,
        );
        if self.filter.mipmap.is_some() {
            self.texture.generate_mipmap();
            self.mipmap_generated = true;
        } else {
            self.mipmap_generated = false;
        }
        self.texture.unbind();
        Ok(())
    }
}
