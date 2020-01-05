use crate::error::{GameError, GameResult};
use winit::window::Icon;
use image::{GenericImageView, Pixel};
use std::path::Path;

pub(crate) fn load_icon<P: AsRef<Path>>(path: P) -> GameResult<Icon> {
    let image = image::open(path).map_err(|error| GameError::IoError(format!("{}", error)))?;
    let (width, height) = image.dimensions();
    let mut rgba = Vec::with_capacity((width * height) as usize * 4);
    for (_, _, pixel) in image.pixels() {
        rgba.extend_from_slice(&pixel.to_rgba().0);
    }
    Icon::from_rgba(rgba, width, height).map_err(|error| GameError::NotSupportedError(format!("{}", error)))
}
