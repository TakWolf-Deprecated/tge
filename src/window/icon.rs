use crate::error::{GameError, GameResult};
use crate::math::Size;
use crate::engine::Engine;
use crate::graphics::{Image, validate_pixels};
use std::path::Path;

pub struct Icon(winit::window::Icon);

impl Icon {
    pub fn new(size: impl Into<Size<u32>>, pixels: Vec<u8>) -> GameResult<Self> {
        let size = size.into();
        validate_pixels(size, &pixels)?;
        let icon = winit::window::Icon::from_rgba(pixels, size.width, size.height)
            .map_err(|error| GameError::InitError(Box::new(error)))?;
        Ok(Self(icon))
    }

    pub fn from_image(image: Image) -> GameResult<Self> {
        let size = image.size();
        let pixels = image.into_pixels();
        Self::new(size, pixels)
    }

    pub fn from_bytes(bytes: &[u8]) -> GameResult<Self> {
        let image = Image::from_bytes(bytes)?;
        Self::from_image(image)
    }

    pub fn load(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<Self> {
        let image = Image::load(engine, path)?;
        Self::from_image(image)
    }
}

impl Into<winit::window::Icon> for Icon {
    fn into(self) -> winit::window::Icon {
        self.0
    }
}
