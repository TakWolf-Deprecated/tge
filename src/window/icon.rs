use crate::error::{GameError, GameResult};
use crate::math::Size;
use crate::graphics::pixel;
use std::path::Path;

pub struct Icon(winit::window::Icon);

impl Icon {

    pub fn new<S: Into<Size<u32>>>(size: S, pixels: Vec<u8>) -> GameResult<Self> {
        let size = size.into();
        pixel::validate_pixels_len(size, &pixels)?;
        let icon = winit::window::Icon::from_rgba(pixels, size.width, size.height)
            .map_err(|error| GameError::NotSupportedError(Box::new(error)))?;
        Ok(Self(icon))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> GameResult<Self> {
        let image = image::open(path)
            .map_err(|error| GameError::InitError(Box::new(error)))?
            .into_rgba();
        let size = Size::new(image.width(), image.height());
        Self::new(size, image.into_raw())
    }

}

impl Into<winit::window::Icon> for Icon {

    fn into(self) -> winit::window::Icon {
        self.0
    }

}
