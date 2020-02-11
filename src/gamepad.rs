mod button;
mod axis;
mod device;

pub use button::GamepadButton;
pub use axis::GamepadAxis;
pub use device::{GamepadId, GamepadDevice};

use crate::error::{GameError, GameResult};
use gilrs::{Gilrs, GilrsBuilder};
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

    pub(crate) fn gilrs(&self) -> &Rc<RefCell<Gilrs>> {
        &self.gilrs
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
