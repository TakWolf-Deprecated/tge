use super::{opengl, Color, PrimitiveType};
use crate::math::{Position, Point, Scale, Region, Angle};
use std::rc::Rc;

#[derive(PartialEq)]
pub struct DrawCommand {
    pub texture: Rc<opengl::Texture>,
    pub primitive_type: PrimitiveType,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct VertexDrawParams {
    pub(crate) primitive_type: Option<PrimitiveType>,
}

impl VertexDrawParams {

    pub fn primitive_type(mut self, primitive_type: PrimitiveType) -> Self {
        self.primitive_type = Some(primitive_type);
        self
    }

}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SpriteDrawParams {
    pub(crate) source: Option<Region>,
    pub(crate) origin: Option<Point>,
    pub(crate) position: Option<Position>,
    pub(crate) rotation: Option<Angle>,
    pub(crate) scale: Option<Scale>,
    pub(crate) colors: Option<[Color; 4]>,
}

impl SpriteDrawParams {

    pub fn source<R: Into<Region>>(mut self, source: R) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn origin<P: Into<Point>>(mut self, origin: P) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn position<P: Into<Position>>(mut self, position: P) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn rotation(mut self, angle: Angle) -> Self {
        self.rotation = Some(angle);
        self
    }

    pub fn scale<S: Into<Scale>>(mut self, scale: S) -> Self {
        self.scale = Some(scale.into());
        self
    }

    pub fn colors(mut self, colors: [Color; 4]) -> Self {
        self.colors = Some(colors);
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        let color = color.into();
        self.colors = Some([color, color, color, color]);
        self
    }

}
