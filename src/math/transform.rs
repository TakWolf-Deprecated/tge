use super::{Vector, Angle};
use glam::{Vec3, Mat4};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform(pub(crate) Mat4);

impl Transform {
    pub fn identity() -> Self {
        Self(Mat4::identity())
    }

    pub fn zero() -> Self {
        Self(Mat4::zero())
    }

    pub fn translate(&self, vector: impl Into<Vector>) -> Self {
        let vector = vector.into();
        Self(Mat4::from_translation(Vec3::new(vector.x, vector.y, 0.0)) * self.0)
    }

    pub fn rotate(&self, angle: impl Into<Angle>) -> Self {
        let angle = angle.into();
        Self(Mat4::from_rotation_z(angle.radians_value()) * self.0)
    }

    pub fn scale(&self, vector: impl Into<Vector>) -> Self {
        let vector = vector.into();
        Self(Mat4::from_scale(Vec3::new(vector.x, vector.y, 1.0)) * self.0)
    }

    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    pub fn apply(&self, transform: impl Into<Transform>) -> Self {
        let transform = transform.into();
        Self(transform.0 * self.0)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
