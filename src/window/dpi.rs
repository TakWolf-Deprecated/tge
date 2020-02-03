use crate::math::{Position, Size};
use winit::dpi::{LogicalPosition, LogicalSize};

impl From<LogicalPosition<f64>> for Position<i32> {

    fn from(logical_position: LogicalPosition<f64>) -> Self {
        Self::new(logical_position.x.round() as i32, logical_position.y.round() as i32)
    }

}

impl Into<LogicalPosition<f64>> for Position<i32> {

    fn into(self) -> LogicalPosition<f64> {
        LogicalPosition::new(self.x as f64, self.y as f64)
    }

}

impl From<LogicalSize<f64>> for Size<u32> {

    fn from(logical_size: LogicalSize<f64>) -> Self {
        Self::new(logical_size.width.round() as u32, logical_size.height.round() as u32)
    }

}

impl Into<LogicalSize<f64>> for Size<u32> {

    fn into(self) -> LogicalSize<f64> {
        LogicalSize::new(self.width as f64, self.height as f64)
    }

}
