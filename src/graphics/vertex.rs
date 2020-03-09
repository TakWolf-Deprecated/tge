use super::Color;
use crate::math::{Vector, Position};

pub const ATTRIBUTE_POSITION_SIZE: usize = 2;
pub const ATTRIBUTE_UV_SIZE: usize = 2;
pub const ATTRIBUTE_COLOR_SIZE: usize = 4;
pub const ATTRIBUTE_STRIDE: usize = ATTRIBUTE_POSITION_SIZE + ATTRIBUTE_UV_SIZE + ATTRIBUTE_COLOR_SIZE;
pub const ATTRIBUTE_OFFSET_0: usize = 0;
pub const ATTRIBUTE_OFFSET_1: usize = ATTRIBUTE_OFFSET_0 + ATTRIBUTE_POSITION_SIZE;
pub const ATTRIBUTE_OFFSET_2: usize = ATTRIBUTE_OFFSET_1 + ATTRIBUTE_UV_SIZE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: Position,
    pub uv: Vector,
    pub color: Color,
}

impl Vertex {

    pub fn new(position: impl Into<Position>, uv: impl Into<Vector>, color: impl Into<Color>) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
            color: color.into(),
        }
    }

}
