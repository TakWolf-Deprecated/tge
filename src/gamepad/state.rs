use super::{GamepadButton, GamepadAxis};
use crate::event::{KeyState, KeyAction};
use std::collections::HashMap;

pub(crate) struct GamepadState {
    connected: bool,
    button_states: HashMap<GamepadButton, KeyState>,
    button_values: HashMap<GamepadButton, f32>,
    axis_values: HashMap<GamepadAxis, f32>,
}

impl GamepadState {

    pub fn new() -> Self {
        Self {
            connected: false,
            button_states: HashMap::new(),
            button_values: HashMap::new(),
            axis_values: HashMap::new(),
        }
    }

    pub fn handle_button_input_event(&mut self, button: GamepadButton, action: KeyAction) {
        self.button_states.insert(button, action.into());
    }

    pub fn handle_button_change_event(&mut self, button: GamepadButton, value: f32) {
        self.button_values.insert(button, value);
    }

    pub fn handle_axis_change_event(&mut self, axis: GamepadAxis, value: f32) {
        self.axis_values.insert(axis, value);
    }

    pub fn clear_states(&mut self) {
        self.button_states.retain(|_, state| match state {
            KeyState::Down | KeyState::Hold => {
                *state = KeyState::Hold;
                true
            }
            KeyState::Up | KeyState::Idle => false,
        });
    }

    pub fn reset(&mut self, connected: bool) {
        self.connected = connected;
        self.button_states.clear();
        self.button_values.clear();
        self.axis_values.clear();
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Down => true,
            _ => false,
        }
    }

    pub fn is_button_hold(&self, button: GamepadButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Down | KeyState::Hold => true,
            _ => false,
        }
    }

    pub fn is_button_up(&self, button: GamepadButton) -> bool {
        match self.button_states.get(&button).unwrap_or(&KeyState::Idle) {
            KeyState::Up => true,
            _ => false,
        }
    }

    pub fn button_value(&self, button: GamepadButton) -> f32 {
        *self.button_values.get(&button).unwrap_or(&0.0)
    }

    pub fn axis_value(&self, axis: GamepadAxis) -> f32 {
        *self.axis_values.get(&axis).unwrap_or(&0.0)
    }

}
