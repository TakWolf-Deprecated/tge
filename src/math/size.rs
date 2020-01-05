use super::{Number, Vector2};
use std::ops::{Mul, Div, MulAssign, DivAssign};
use winit::dpi::LogicalSize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size<N: Number = f32> {
    pub width: N,
    pub height: N,
}

impl<N: Number> Size<N> {

    pub fn new(width: N, height: N) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self::new(N::zero(), N::zero())
    }

    pub fn set(&mut self, width: N, height: N) {
        self.width = width;
        self.height = height;
    }

    pub fn set_with(&mut self, other: &Self) {
        self.width = other.width;
        self.height = other.height;
    }

}

impl<N: Number> Mul<Vector2<N>> for Size<N> {

    type Output = Self;

    fn mul(self, vec: Vector2<N>) -> Self::Output {
        Self::new(self.width * vec.x, self.height * vec.y)
    }

}

impl<N: Number> Div<Vector2<N>> for Size<N> {

    type Output = Self;

    fn div(self, vec: Vector2<N>) -> Self::Output {
        Self::new(self.width / vec.x, self.height / vec.y)
    }

}

impl<N: Number> MulAssign<Vector2<N>> for Size<N> {

    fn mul_assign(&mut self, vec: Vector2<N>) {
        self.width *= vec.x;
        self.height *= vec.y;
    }

}

impl<N: Number> DivAssign<Vector2<N>> for Size<N> {

    fn div_assign(&mut self, vec: Vector2<N>) {
        self.width /= vec.x;
        self.height /= vec.y;
    }

}

impl<N: Number> Mul<N> for Size<N> {

    type Output = Self;

    fn mul(self, value: N) -> Self::Output {
        Self::new(self.width * value, self.height * value)
    }

}

impl<N: Number> Div<N> for Size<N> {

    type Output = Self;

    fn div(self, value: N) -> Self::Output {
        Self::new(self.width / value, self.height / value)
    }

}

impl<N: Number> MulAssign<N> for Size<N> {

    fn mul_assign(&mut self, value: N) {
        self.width *= value;
        self.height *= value;
    }

}

impl<N: Number> DivAssign<N> for Size<N> {

    fn div_assign(&mut self, value: N) {
        self.width /= value;
        self.height /= value;
    }

}

impl<N: Number> From<(N, N)> for Size<N> {

    fn from((width, height): (N, N)) -> Self {
        Self::new(width, height)
    }

}

impl<N: Number> Into<(N, N)> for Size<N> {

    fn into(self) -> (N, N) {
        (self.width, self.height)
    }

}

impl From<LogicalSize> for Size<u32> {

    fn from(size: LogicalSize) -> Self {
        Self::new(size.width.round() as u32, size.height.round() as u32)
    }

}

impl Into<LogicalSize> for Size<u32> {

    fn into(self) -> LogicalSize {
        LogicalSize::new(self.width as f64, self.height as f64)
    }

}

#[cfg(test)]
mod tests {

    use super::Size;
    use crate::math::Vector2;

    #[test]
    fn operator() {
        let mut size = Size::<f32>::new(10.0, 20.0);
        size = size * Vector2::<f32>::new(2.0, 2.0);
        assert_eq!(size, Size::<f32>::new(20.0, 40.0));
        size = size / Vector2::<f32>::new(10.0, 5.0);
        assert_eq!(size, Size::<f32>::new(2.0, 8.0));
        size *= Vector2::<f32>::new(4.0, 2.0);
        assert_eq!(size, Size::<f32>::new(8.0, 16.0));
        size /= Vector2::<f32>::new(8.0, 4.0);
        assert_eq!(size, Size::<f32>::new(1.0, 4.0));
        size = size * 6.0f32;
        assert_eq!(size, Size::<f32>::new(6.0, 24.0));
        size = size / 3.0f32;
        assert_eq!(size, Size::<f32>::new(2.0, 8.0));
        size *= 5.0f32;
        assert_eq!(size, Size::<f32>::new(10.0, 40.0));
        size /= 10.0f32;
        assert_eq!(size, Size::<f32>::new(1.0, 4.0));
    }

}
