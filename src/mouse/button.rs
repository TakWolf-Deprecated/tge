
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}

impl From<winit::event::MouseButton> for MouseButton {

    fn from(button: winit::event::MouseButton) -> Self {
        match button {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            winit::event::MouseButton::Other(button_id) => MouseButton::Other(button_id as u32),
        }
    }

}
