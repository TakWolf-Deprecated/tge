use super::{Number, Position, Size};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Region<N: Number = f32> {
    pub x: N,
    pub y: N,
    pub width: N,
    pub height: N,
}

pub type Viewport<N> = Region<N>;

impl<N: Number> Region<N> {

    pub fn new(x: N, y: N, width: N, height: N) -> Self {
        Self { x, y, width, height}
    }

    pub fn edge(left: N, right: N, top: N, bottom: N) -> Self {
        Self::new(left, top, right - left, bottom - top)
    }

    pub fn zero() -> Self {
        Self::new(N::zero(), N::zero(), N::zero(), N::zero())
    }

    pub fn set(&mut self, x: N, y: N, width: N, height: N) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn set_with(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
        self.width = other.width;
        self.height = other.height;
    }

    pub fn left(&self) -> N {
        self.x
    }

    pub fn set_left(&mut self, left: N) {
        self.x = left;
    }

    pub fn right(&self) -> N {
        self.x + self.width
    }

    pub fn set_right(&mut self, right: N) {
        self.width = right - self.x;
    }

    pub fn top(&self) -> N {
        self.y
    }

    pub fn set_top(&mut self, top: N) {
        self.y = top;
    }

    pub fn bottom(&self) -> N {
        self.y + self.height
    }

    pub fn set_bottom(&mut self, bottom: N) {
        self.height = bottom - self.y;
    }

    pub fn set_edge(&mut self, left: N, right: N, top: N, bottom: N) {
        self.set_left(left);
        self.set_right(right);
        self.set_top(top);
        self.set_bottom(bottom);
    }

    pub fn top_left(&self) -> Position<N> {
        Position::new(self.x, self.y)
    }

    pub fn top_right(&self) -> Position<N> {
        Position::new(self.x + self.width, self.y)
    }

    pub fn bottom_left(&self) -> Position<N> {
        Position::new(self.x, self.y + self.height)
    }

    pub fn bottom_right(&self) -> Position<N> {
        Position::new(self.x + self.width, self.y + self.height)
    }

    pub fn position(&self) -> Position<N> {
        Position::new(self.x, self.y)
    }

    pub fn size(&self) -> Size<N> {
        Size::new(self.width, self.height)
    }

}

impl<N: Number> From<(N, N, N, N)> for Region<N> {

    fn from((x, y, width, height): (N, N, N, N)) -> Self {
        Self::new(x, y, width, height)
    }

}

impl<N: Number> Into<(N, N, N, N)> for Region<N> {

    fn into(self) -> (N, N, N, N) {
        (self.x, self.y, self.width, self.height)
    }

}
