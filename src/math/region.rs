use super::{Number, Position, Size};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Region<N: Number = f32> {
    pub x: N,
    pub y: N,
    pub width: N,
    pub height: N,
}

pub type Viewport<N = f32> = Region<N>;

impl<N: Number> Region<N> {
    pub fn new(x: N, y: N, width: N, height: N) -> Self {
        Self { x, y, width, height }
    }

    pub fn position_size(position: Position<N>, size: Size<N>) -> Self {
        Self::new(position.x, position.y, size.width, size.height)
    }

    pub fn min_max(min: Position<N>, max: Position<N>) -> Self {
        Self::new(min.x, min.y, max.x - min.x, max.y - min.y)
    }

    pub fn edge(left: N, right: N, top: N, bottom: N) -> Self {
        Self::new(left, top, right - left, bottom - top)
    }

    pub fn zero() -> Self {
        Self::new(N::zero(), N::zero(), N::zero(), N::zero())
    }

    pub fn none() -> Option<Self> {
        None
    }

    pub fn position(&self) -> Position<N> {
        Position::new(self.x, self.y)
    }

    pub fn set_position(&mut self, position: Position<N>) {
        self.x = position.x;
        self.y = position.y;
    }

    pub fn size(&self) -> Size<N> {
        Size::new(self.width, self.height)
    }

    pub fn set_size(&mut self, size: Size<N>) {
        self.width = size.width;
        self.height = size.height;
    }

    pub fn min_x(&self) -> N {
        self.x
    }

    pub fn set_min_x(&mut self, min_x: N) {
        self.width += self.x - min_x;
        self.x = min_x;
    }

    pub fn min_y(&self) -> N {
        self.y
    }

    pub fn set_min_y(&mut self, min_y: N) {
        self.height += self.y - min_y;
        self.y = min_y;
    }

    pub fn min(&self) -> Position<N> {
        Position::new(self.min_x(), self.min_y())
    }

    pub fn set_min(&mut self, min: Position<N>) {
        self.set_min_x(min.x);
        self.set_min_y(min.y);
    }

    pub fn max_x(&self) -> N {
        self.x + self.width
    }

    pub fn set_max_x(&mut self, max_x: N) {
        self.width = max_x - self.x;
    }

    pub fn max_y(&self) -> N {
        self.y + self.height
    }

    pub fn set_max_y(&mut self, max_y: N) {
        self.height = max_y - self.y;
    }

    pub fn max(&self) -> Position<N> {
        Position::new(self.max_x(), self.max_y())
    }

    pub fn set_max(&mut self, max: Position<N>) {
        self.set_max_x(max.x);
        self.set_max_y(max.y);
    }

    pub fn left(&self) -> N {
        self.x
    }

    pub fn set_left(&mut self, left: N) {
        self.width += self.x - left;
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
        self.height += self.y - top;
        self.y = top;
    }

    pub fn bottom(&self) -> N {
        self.y + self.height
    }

    pub fn set_bottom(&mut self, bottom: N) {
        self.height = bottom - self.y;
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

#[cfg(test)]
mod tests {
    use super::Region;
    use crate::math::{Position, Size};

    #[test]
    fn test_create() {
        let region = Region::<f32>::new(10.0, 20.0, 100.0, 150.0);
        assert_eq!(region, Region::<f32>::position_size(Position::<f32>::new(10.0, 20.0), Size::<f32>::new(100.0, 150.0)));
        assert_eq!(region, Region::<f32>::min_max(Position::<f32>::new(10.0, 20.0), Position::<f32>::new(110.0, 170.0)));
        assert_eq!(region, Region::<f32>::edge(10.0, 110.0, 20.0, 170.0));
        assert_eq!(region.position(), Position::<f32>::new(10.0, 20.0));
        assert_eq!(region.size(), Size::<f32>::new(100.0, 150.0));
        assert_eq!(region.min_x(), 10.0f32);
        assert_eq!(region.min_y(), 20.0f32);
        assert_eq!(region.max_x(), 110.0f32);
        assert_eq!(region.max_y(), 170.0f32);
        assert_eq!(region.min(), Position::<f32>::new(10.0, 20.0));
        assert_eq!(region.max(), Position::<f32>::new(110.0, 170.0));
        assert_eq!(region.left(), 10.0f32);
        assert_eq!(region.right(), 110.0f32);
        assert_eq!(region.top(), 20.0f32);
        assert_eq!(region.bottom(), 170.0f32);
        assert_eq!(region.top_left(), Position::<f32>::new(10.0, 20.0));
        assert_eq!(region.top_right(), Position::<f32>::new(110.0, 20.0));
        assert_eq!(region.bottom_left(), Position::<f32>::new(10.0, 170.0));
        assert_eq!(region.bottom_right(), Position::<f32>::new(110.0, 170.0));
        assert_eq!(region.position(), region.min());
        assert_eq!(region.position(), region.top_left());
        assert_eq!(region.min(), region.top_left());
        assert_eq!(region.max(), region.bottom_right());
    }
}
