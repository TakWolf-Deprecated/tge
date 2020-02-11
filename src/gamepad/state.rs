use super::{GamepadButton, GamepadAxis};
use crate::event::KeyState;
use std::collections::HashMap;

pub(crate) struct GamepadState {
    button_states: HashMap<GamepadButton, KeyState>,
    button_values: HashMap<GamepadButton, f32>,
    axis_values: HashMap<GamepadAxis, f32>,
}

impl GamepadState {

    pub fn new() -> Self {
        Self {
            button_states: HashMap::new(),
            button_values: HashMap::new(),
            axis_values: HashMap::new(),
        }
    }

}
