use crate::math::{Position, Size, Delta};
use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;
use crate::touch::TouchPhase;
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
    WindowMove(Position),
    WindowFocusChange(bool),
    ReceiveChar(char),
    KeyboardInput {
        key: KeyCode,
        action: KeyAction,
    },
    MouseMove(Position),
    MouseEnterWindow,
    MouseLeaveWindow,
    MouseWheelScroll(Delta),
    MouseInput {
        button: MouseButton,
        action: KeyAction,
    },
    Touch {
        id: u64,
        phase: TouchPhase,
        position: Position,
    },
    TouchpadScroll(Delta),
    TouchpadPress {
        pressure: f32,
        click_stage: i64,
    },

    // TODO

}
