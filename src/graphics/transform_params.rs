use crate::math::{Position, Point, Scale, Angle};
use glam::{Vec3, Quat, Mat4};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TransformParams {
    pub origin: Option<Point>,
    pub position: Option<Position>,
    pub rotation: Option<Angle>,
    pub scale: Option<Scale>,
}

impl TransformParams {

    pub fn origin(mut self, origin: impl Into<Point>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn position(mut self, position: impl Into<Position>) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn rotation(mut self, angle: Angle) -> Self {
        self.rotation = Some(angle);
        self
    }

    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.scale = Some(scale.into());
        self
    }

    pub(crate) fn matrix(&self) -> Mat4 {
        let position = self.position.map(|position| Vec3::new(position.x, position.y, 0.0)).unwrap_or_else(|| Vec3::zero());
        let rotation = self.rotation.map(|angle| Quat::from_rotation_z(angle.radians_value())).unwrap_or_else(|| Quat::from_rotation_z(0.0));
        let scale = self.scale.map(|scale| Vec3::new(scale.x, scale.y, 1.0)).unwrap_or_else(|| Vec3::one());
        Mat4::from_scale_rotation_translation(scale, rotation, position)
    }

}
