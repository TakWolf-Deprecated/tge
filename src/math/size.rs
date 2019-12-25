use crate::math::number::Number;
use crate::math::vector::Vector2;
use std::ops::{Mul, Div, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size<N: Number> {
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
