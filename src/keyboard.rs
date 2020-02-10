mod code;

pub use code::KeyCode;

use crate::error::GameResult;
use crate::event::{KeyState, KeyAction};
use std::collections::HashMap;

pub struct Keyboard {
    key_states: HashMap<KeyCode, KeyState>,
}

impl Keyboard {

    pub(crate) fn new(_: KeyboardConfig) -> GameResult<Self> {
        Ok(Self {
            key_states: HashMap::new(),
        })
    }

    pub(crate) fn handle_input_event(&mut self, key: KeyCode, scan_code: u32, action: KeyAction) {
        self.key_states.insert(key, action.into());
        self.key_states.insert(KeyCode::Other(scan_code), action.into());
    }

    pub(crate) fn clear_states(&mut self) {
        self.key_states.retain(|_, state| match state {
            KeyState::Down | KeyState::Hold => {
                *state = KeyState::Hold;
                true
            }
            KeyState::Up | KeyState::Idle => false,
        });
    }
    
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Down => true,
            _ => false,
        }
    }

    pub fn is_key_hold(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Down | KeyState::Hold => true,
            _ => false,
        }
    }

    pub fn is_key_up(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Up => true,
            _ => false,
        }
    }

}

#[derive(Debug, Clone)]
pub struct KeyboardConfig {}

impl KeyboardConfig {

    pub fn new() -> Self {
        Self {}
    }

}
