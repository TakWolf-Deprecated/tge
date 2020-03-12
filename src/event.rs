use crate::math::{Position, Size, Delta};
use crate::keyboard::{KeyCode, ModifiersState};
use crate::mouse::MouseButton;
use crate::touch::TouchPhase;
use crate::gamepad::{GamepadButton, GamepadAxis, GamepadId};
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
        repeated: bool,
    },
    ModifiersChange(ModifiersState),
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
    TouchpadScroll {
        delta: Delta,
        phase: TouchPhase,
    },
    TouchpadPress {
        pressure: f32,
        click_stage: i64,
    },
    GamepadConnect(GamepadId),
    GamepadDisconnect(GamepadId),
    GamepadButtonInput {
        id: GamepadId,
        button: GamepadButton,
        action: KeyAction,
    },
    GamepadButtonChange {
        id: GamepadId,
        button: GamepadButton,
        value: f32,
    },
    GamepadAxisChange {
        id: GamepadId,
        axis: GamepadAxis,
        value: f32,
    },
}
