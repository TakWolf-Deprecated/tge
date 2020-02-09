mod cursor;
mod button;

pub use cursor::CursorIcon;
pub use button::MouseButton;

use crate::error::{GameError, GameResult};
use crate::math::Position;
use crate::event::KeyState;
use winit::window::Window;
use winit::dpi::LogicalPosition;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::rc::Rc;
use std::collections::HashMap;

pub struct Mouse {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
    cursor_icon: CursorIcon,
    cursor_visible: bool,
    position: Position,
    inside_window: bool,
    wheel_delta: f32,
    button_states: HashMap<MouseButton, KeyState>,
}

impl Mouse {

    pub(crate) fn new(mouse_config: MouseConfig, context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>) -> GameResult<Self> {
        let window = context_wrapper.window();
        window.set_cursor_icon(mouse_config.cursor_icon.into());
        window.set_cursor_visible(mouse_config.cursor_visible);
        Ok(Self {
            context_wrapper,
            cursor_icon: mouse_config.cursor_icon,
            cursor_visible: mouse_config.cursor_visible,
            position: Position::zero(),
            inside_window: false,
            wheel_delta: 0.0,
            button_states: HashMap::new(),
        })
    }

    fn window(&self) -> &Window {
        self.context_wrapper.window()
    }

    pub(crate) fn reset_states(&mut self) {
        self.button_states.retain(|_, state| match state {
            KeyState::Down | KeyState::Hold => {
                *state = KeyState::Hold;
                true
            },
            KeyState::Up | KeyState::Idle => false,
        });
        self.wheel_delta = 0.0;
    }

    pub fn cursor_icon(&self) -> CursorIcon {
        self.cursor_icon
    }

    pub fn set_cursor_icon(&mut self, cursor_icon: CursorIcon) {
        self.window().set_cursor_icon(cursor_icon.into());
        self.cursor_icon = cursor_icon;
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn set_cursor_visible(&mut self, cursor_visible: bool) {
        self.window().set_cursor_visible(cursor_visible);
        self.cursor_visible = cursor_visible;
    }

    pub fn position(&self) -> Option<Position> {
        if self.inside_window {
            Some(self.position)
        } else {
            None
        }
    }

    pub fn last_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) -> GameResult {
        self.window().set_cursor_position(LogicalPosition::new(position.x, position.y))
            .map_err(|error| GameError::NotSupportedError(format!("{}", error)))?;
        self.position = position;
        Ok(())
    }

    pub fn is_inside_window(&self) -> bool {
        self.inside_window
    }

    pub fn wheel_delta(&self) -> f32 {
        self.wheel_delta
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Down => true,
            _ => false,
        }
    }

    pub fn is_button_hold(&self, button: MouseButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Down | KeyState::Hold => true,
            _ => false,
        }
    }

    pub fn is_button_up(&self, button: MouseButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Up => true,
            _ => false,
        }
    }

}

#[derive(Debug, Clone)]
pub struct MouseConfig {
    cursor_icon: CursorIcon,
    cursor_visible: bool,
}

impl MouseConfig {

    pub fn new() -> Self {
        Self {
            cursor_icon: CursorIcon::default(),
            cursor_visible: true,
        }
    }

    pub fn cursor_icon(mut self, cursor_icon: CursorIcon) -> Self {
        self.cursor_icon = cursor_icon;
        self
    }

    pub fn cursor_visible(mut self, cursor_visible: bool) -> Self {
        self.cursor_visible = cursor_visible;
        self
    }

}
