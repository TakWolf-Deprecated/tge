use super::{Number, Vector};
use std::ops::{Mul, Div, MulAssign, DivAssign};

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

    pub fn none() -> Option<Self> {
        None
    }
}

impl<N: Number> Mul<Vector<N>> for Size<N> {
    type Output = Self;

    fn mul(self, vec: Vector<N>) -> Self::Output {
        Self::new(self.width * vec.x, self.height * vec.y)
    }
}

impl<N: Number> Div<Vector<N>> for Size<N> {
    type Output = Self;

    fn div(self, vec: Vector<N>) -> Self::Output {
        Self::new(self.width / vec.x, self.height / vec.y)
    }
}

impl<N: Number> MulAssign<Vector<N>> for Size<N> {
    fn mul_assign(&mut self, vec: Vector<N>) {
        self.width *= vec.x;
        self.height *= vec.y;
    }
}

impl<N: Number> DivAssign<Vector<N>> for Size<N> {
    fn div_assign(&mut self, vec: Vector<N>) {
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

#[cfg(test)]
mod tests {
    use super::Size;
    use crate::math::Vector;

    #[test]
    fn test_operator() {
        let mut size = Size::<f32>::new(10.0, 20.0);
        size = size * Vector::<f32>::new(2.0, 2.0);
        assert_eq!(size, Size::<f32>::new(20.0, 40.0));
        size = size / Vector::<f32>::new(10.0, 5.0);
        assert_eq!(size, Size::<f32>::new(2.0, 8.0));
        size *= Vector::<f32>::new(4.0, 2.0);
        assert_eq!(size, Size::<f32>::new(8.0, 16.0));
        size /= Vector::<f32>::new(8.0, 4.0);
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
