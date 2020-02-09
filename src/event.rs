use crate::math::{Position, Size, Delta};
use crate::mouse::MouseButton;
use winit::event::ElementState;

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

impl From<ElementState> for KeyAction {

    fn from(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => KeyAction::Down,
            ElementState::Released => KeyAction::Up,
        }
    }

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
    MouseTouchpadScroll(Delta),
    MouseInput {
        button: MouseButton,
        action: KeyAction,
    },
}
