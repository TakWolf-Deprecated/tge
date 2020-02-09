use crate::math::{Position, Size, Delta};
use crate::mouse::MouseButton;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum KeyState {
    Down,
    Hold,
    Up,
    Idle,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum KeyAction {
    Down,
    Up,
}

impl Into<KeyState> for KeyAction {

    fn into(self) -> KeyState {
        match self {
            KeyAction::Down => KeyState::Down,
            KeyAction::Up => KeyState::Up,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    AppSuspend,
    AppResume,
    WindowClose,
    WindowResize(Size<u32>),
    WindowMove(Position<i32>),
    WindowFocusChange(bool),
    ReceiveChar(char),

    // TODO
    KeyboardInput,
    // TODO

    MouseMove(Position),
    MouseEnterWindow,
    MouseLeaveWindow,
    MouseWheelScroll(Delta),
    MouseButtonInput {
        button: MouseButton,
        action: KeyAction,
    },
}
