use crate::error::{GameError, GameResult};
use image::{GenericImageView, Pixel};
use std::path::Path;

pub struct Icon(winit::window::Icon);

impl Icon {

    pub fn load<P: AsRef<Path>>(path: P) -> GameResult<Self> {
        let image = image::open(path)
            .map_err(|error| GameError::IoError(format!("{}", error)))?;
        let (width, height) = image.dimensions();
        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
        for (_, _, pixel) in image.pixels() {
            rgba.extend_from_slice(&pixel.to_rgba().0);
        }
        let icon = winit::window::Icon::from_rgba(rgba, width, height)
            .map_err(|error| GameError::NotSupportedError(format!("{}", error)))?;
        Ok(Self(icon))
    }

}

impl Into<winit::window::Icon> for Icon {

    fn into(self) -> winit::window::Icon {
        self.0
    }

}
