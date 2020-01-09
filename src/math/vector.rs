use super::Number;
use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<N: Number = f32> {
    pub x: N,
    pub y: N,
}

pub type Position<N = f32> = Vector2<N>;
pub type Point<N = f32> = Vector2<N>;
pub type Scale<N = f32> = Vector2<N>;

impl<N: Number> Vector2<N> {

    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(N::zero(), N::zero())
    }

    pub fn set(&mut self, x: N, y: N) {
        self.x = x;
        self.y = y;
    }

    pub fn set_with(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
    }

}

impl<N: Number> Add for Vector2<N> {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }

}

impl<N: Number> Sub for Vector2<N> {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }

}

impl<N: Number> AddAssign for Vector2<N> {

    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }

}

impl<N: Number> SubAssign for Vector2<N> {

    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }

}

impl<N: Number> From<(N, N)> for Vector2<N> {

    fn from((x, y): (N, N)) -> Self {
        Self::new(x, y)
    }

}

impl<N: Number> Into<(N, N)> for Vector2<N> {

    fn into(self) -> (N, N) {
        (self.x, self.y)
    }

}

#[cfg(test)]
mod tests {

    use super::Vector2;

    #[test]
    fn operator() {
        let mut vec = Vector2::<f32>::new(100.0, 50.0);
        vec = vec + Vector2::<f32>::new(40.0, 20.0);
        assert_eq!(vec, Vector2::<f32>::new(140.0, 70.0));
        vec = vec - Vector2::<f32>::new(40.0, 70.0);
        assert_eq!(vec, Vector2::<f32>::new(100.0, 0.0));
        vec += Vector2::<f32>::new(20.0, 50.0);
        assert_eq!(vec, Vector2::new(120.0, 50.0));
        vec -= Vector2::<f32>::new(80.0, 10.0);
        assert_eq!(vec, Vector2::new(40.0, 40.0));
    }

}
