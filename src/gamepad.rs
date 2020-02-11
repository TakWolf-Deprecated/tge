mod button;
mod axis;
mod state;
mod device;
mod power;

use state::GamepadState;

pub use button::GamepadButton;
pub use axis::GamepadAxis;
pub use device::{GamepadId, GamepadDevice};
pub use power::PowerInfo;

use crate::error::{GameError, GameResult};
use crate::event::KeyAction;
use gilrs::{Gilrs, GilrsBuilder, Event};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Gamepad {
    gilrs: Rc<RefCell<Gilrs>>,
    connected_states: HashMap<GamepadId, Rc<RefCell<GamepadState>>>,
    disconnected_states: HashMap<GamepadId, Rc<RefCell<GamepadState>>>,
}

impl Gamepad {

    pub(crate) fn new(gamepad_config: GamepadConfig) -> GameResult<Self> {
        let gilrs = GilrsBuilder::new()
            .set_axis_to_btn(gamepad_config.axis_to_button_down_value, gamepad_config.axis_to_button_up_value)
            .build()
            .map_err(|error| GameError::InitError(format!("{}", error)))?;
        Ok(Self {
            gilrs: Rc::new(RefCell::new(gilrs)),
            connected_states: HashMap::new(),
            disconnected_states: HashMap::new(),
        })
    }

    pub(crate) fn pump_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        while let Some(event) = self.gilrs.borrow_mut().next_event() {
            events.push(event);
        }
        events
    }

    pub(crate) fn handle_connect_event(&mut self, id: GamepadId) {
        let state = self.disconnected_states.remove(&id).unwrap_or_else(|| Rc::new(RefCell::new(GamepadState::new())));
        state.borrow_mut().reset(true);
        self.connected_states.insert(id, state);
    }

    pub(crate) fn handle_disconnect_event(&mut self, id: GamepadId) {
        let state = self.connected_states.remove(&id).unwrap_or_else(|| Rc::new(RefCell::new(GamepadState::new())));
        state.borrow_mut().reset(false);
        self.disconnected_states.insert(id, state);
    }

    pub(crate) fn handle_button_input_event(&mut self, id: GamepadId, button: GamepadButton, action: KeyAction) {
        if let Some(state) = self.connected_states.get(&id) {
            state.borrow_mut().handle_button_input_event(button, action);
        }
    }

    pub(crate) fn handle_button_change_event(&mut self, id: GamepadId, button: GamepadButton, value: f32) {
        if let Some(state) = self.connected_states.get(&id) {
            state.borrow_mut().handle_button_change_event(button, value);
        }
    }

    pub(crate) fn handle_axis_change_event(&mut self, id: GamepadId, axis: GamepadAxis, value: f32) {
        if let Some(state) = self.connected_states.get(&id) {
            state.borrow_mut().handle_axis_change_event(axis, value);
        }
    }

    pub(crate) fn clear_states(&mut self) {
        for (_, state) in &self.connected_states {
            state.borrow_mut().clear_states();
        }
    }

    pub fn device(&self, id: GamepadId) -> GamepadDevice {
        let mut state = self.connected_states.get(&id);
        if state.is_none() {
            state = self.disconnected_states.get(&id);
        }
        let state = state.expect("can not find gamepad state");
        GamepadDevice::new(self.gilrs.clone(), id, state.clone())
    }

    pub fn connected_device(&self, id: GamepadId) -> Option<GamepadDevice> {
        self.connected_states.get(&id).map(|state| {
            GamepadDevice::new(self.gilrs.clone(), id, state.clone())
        })
    }

    pub fn connected_devices(&self) -> Vec<GamepadDevice> {
        let mut devices = Vec::new();
        for (id, state) in &self.connected_states {
            devices.push(GamepadDevice::new(self.gilrs.clone(), *id, state.clone()))
        }
        devices
    }

    pub fn is_connected(&self, id: GamepadId) -> bool {
        self.connected_states.contains_key(&id)
    }

    pub fn is_button_down(&self, id: GamepadId, button: GamepadButton) -> bool {
        self.connected_states.get(&id)
            .map(|state| state.borrow().is_button_down(button))
            .unwrap_or(false)
    }

    pub fn is_button_hold(&self, id: GamepadId, button: GamepadButton) -> bool {
        self.connected_states.get(&id)
            .map(|state| state.borrow().is_button_hold(button))
            .unwrap_or(false)
    }

    pub fn is_button_up(&self, id: GamepadId, button: GamepadButton) -> bool {
        self.connected_states.get(&id)
            .map(|state| state.borrow().is_button_up(button))
            .unwrap_or(false)
    }

    pub fn button_value(&self, id: GamepadId, button: GamepadButton) -> f32 {
        self.connected_states.get(&id)
            .map(|state| state.borrow().button_value(button))
            .unwrap_or(0.0)
    }

    pub fn axis_value(&self, id: GamepadId, axis: GamepadAxis) -> f32 {
        self.connected_states.get(&id)
            .map(|state| state.borrow().axis_value(axis))
            .unwrap_or(0.0)
    }

}

#[derive(Debug, Clone)]
pub struct GamepadConfig {
    axis_to_button_down_value: f32,
    axis_to_button_up_value: f32,
}

impl GamepadConfig {

    pub fn new() -> Self {
        Self {
            axis_to_button_down_value: 0.75,
            axis_to_button_up_value: 0.65,
        }
    }

    pub fn axis_to_button_down_value(mut self, value: f32) -> Self {
        self.axis_to_button_down_value = value;
        self
    }

    pub fn axis_to_button_up_value(mut self, value: f32) -> Self {
        self.axis_to_button_up_value = value;
        self
    }

}
