use super::{opengl, Filter, Wrap};
use crate::error::{GameError, GameResult};
use crate::math::{Size, Region};
use crate::engine::Engine;
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

    pub fn new(engine: &mut Engine, size: Size<u32>, pixels: Option<&[u8]>) -> GameResult<Self> {
        let filter = engine.graphics().default_filter();
        let generate_mipmap = filter.mipmap.is_some();
        let wrap = engine.graphics().default_wrap();
        let texture = opengl::Texture::new(engine.graphics().gl().clone())
            .map_err(|error| GameError::InitError(error.into()))?;
        texture.bind();
        texture.init_image_2d(size.width, size.height, pixels);
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

    pub fn from_bytes(engine: &mut Engine, bytes: &[u8]) -> GameResult<Self> {
        let image = image::load_from_memory(bytes)
            .map_err(|error| GameError::InitError(Box::new(error)))?
            .into_rgba();
        let size = Size::new(image.width(), image.height());
        Self::new(engine, size, Some(image.into_raw().as_slice()))
    }

    pub fn load<P: AsRef<Path>>(engine: &mut Engine, path: P) -> GameResult<Self> {
        let image = image::open(path)
            .map_err(|error| GameError::InitError(Box::new(error)))?
            .into_rgba();
        let size = Size::new(image.width(), image.height());
        Self::new(engine, size, Some(image.into_raw().as_slice()))
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

    pub fn init_pixels(&mut self, size: Size<u32>, pixels: Option<&[u8]>) {
        self.texture.bind();
        self.texture.init_image_2d(size.width, size.height, pixels);
        self.size = size;
        if self.filter.mipmap.is_some() {
            self.texture.generate_mipmap();
            self.mipmap_generated = true;
        } else {
            self.mipmap_generated = false;
        }
        self.texture.unbind();
    }

    pub fn update_pixels(&mut self, region: Region<u32>, pixels: Option<&[u8]>) {
        self.texture.bind();
        self.texture.sub_image_2d(
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
    }

}
