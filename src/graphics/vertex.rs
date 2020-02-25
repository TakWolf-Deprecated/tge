use super::Color;
use crate::math::Vector2;

pub(crate) const ATTRIBUTE_POSITION_SIZE: usize = 2;
pub(crate) const ATTRIBUTE_UV_SIZE: usize = 2;
pub(crate) const ATTRIBUTE_COLOR_SIZE: usize = 4;
pub(crate) const ATTRIBUTE_STRIDE: usize = ATTRIBUTE_POSITION_SIZE + ATTRIBUTE_UV_SIZE + ATTRIBUTE_COLOR_SIZE;
pub(crate) const ATTRIBUTE_OFFSET_0: usize = 0;
pub(crate) const ATTRIBUTE_OFFSET_1: usize = ATTRIBUTE_OFFSET_0 + ATTRIBUTE_POSITION_SIZE;
pub(crate) const ATTRIBUTE_OFFSET_2: usize = ATTRIBUTE_OFFSET_1 + ATTRIBUTE_UV_SIZE;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector2,
    pub uv: Vector2,
    pub color: Color,
}

impl Vertex {

    pub fn new<V: Into<Vector2>, C: Into<Color>>(position: V, uv: V, color: C) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
            color: color.into(),
        }
    }

}
