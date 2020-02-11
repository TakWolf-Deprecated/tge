mod button;
mod axis;
mod device;
mod power;

pub use button::GamepadButton;
pub use axis::GamepadAxis;
pub use device::{GamepadId, GamepadDevice};
pub use power::PowerInfo;

use crate::error::{GameError, GameResult};
use crate::event::KeyAction;
use gilrs::{Gilrs, GilrsBuilder, Event};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Gamepad {
    gilrs: Rc<RefCell<Gilrs>>,
}

impl Gamepad {

    pub(crate) fn new(gamepad_config: GamepadConfig) -> GameResult<Self> {
        let gilrs = GilrsBuilder::new()
            .set_axis_to_btn(gamepad_config.axis_to_button_down_value, gamepad_config.axis_to_button_up_value)
            .build()
            .map_err(|error| GameError::InitError(format!("{}", error)))?;
        Ok(Self {
            gilrs: Rc::new(RefCell::new(gilrs)),
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
        // TODO
    }

    pub(crate) fn handle_disconnect_event(&mut self, id: GamepadId) {
        // TODO
    }

    pub(crate) fn handle_button_input_event(&mut self, id: GamepadId, button: GamepadButton, action: KeyAction) {
        // TODO
    }

    pub(crate) fn handle_button_change_event(&mut self, id: GamepadId, button: GamepadButton, value: f32) {
        // TODO
    }

    pub(crate) fn handle_axis_change_event(&mut self, id: GamepadId, axis: GamepadAxis, value: f32) {
        // TODO
    }

    pub(crate) fn clear_states(&mut self) {
        // TODO
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
