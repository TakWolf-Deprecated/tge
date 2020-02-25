use crate::error::{GameError, GameResult};
use crate::math::Size;

pub fn validate_pixels_len(size: Size<u32>, pixels: &[u8]) -> GameResult {
    if (size.width * size.height * 4) as usize == pixels.len() {
        Ok(())
    } else {
        Err(GameError::RuntimeError("illegal pixels length".into()))
    }
}
