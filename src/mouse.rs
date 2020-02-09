mod cursor;
mod button;

pub use cursor::CursorIcon;
pub use button::MouseButton;

use crate::error::GameResult;
use crate::math::Position;
use crate::event::KeyState;
use winit::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::rc::Rc;
use std::collections::HashMap;

pub struct Mouse {
    context_wrapper: Rc<ContextWrapper<PossiblyCurrent, Window>>,
    cursor_icon: CursorIcon,
    cursor_visible: bool,
    position: Position,
    inside_window: bool,
    button_states: HashMap<MouseButton, KeyState>,
    wheel_delta: f32,
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
            button_states: HashMap::new(),
            wheel_delta: 0.0,
        })
    }

    fn window(&self) -> &Window {
        self.context_wrapper.window()
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
