use crate::math::{Position, Size};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum KeyAction {
    Down,
    Up,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AppSuspend,
    AppResume,
    WindowClose,
    WindowResize(Size<u32>),
    WindowMove(Position<i32>),
    WindowFocusChange(bool),
    // TODO
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum KeyState {
    Down,
    Hold,
    Up,
    Idle,
}
