use super::{GamepadState, PowerInfo};
use gilrs::Gilrs;
use std::rc::Rc;
use std::cell::RefCell;

pub type GamepadId = gilrs::GamepadId;

pub struct GamepadDevice {
    gilrs: Rc<RefCell<Gilrs>>,
    id: GamepadId,
    name: String,
    state: Rc<RefCell<GamepadState>>,
}

impl GamepadDevice {

    pub(crate) fn new(gilrs: Rc<RefCell<Gilrs>>, id: GamepadId, state: Rc<RefCell<GamepadState>>) -> Self {
        let name = gilrs.borrow().gamepad(id).name().to_owned();
        Self { gilrs, id, name, state }
    }

    pub fn id(&self) -> GamepadId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn uuid(&self) -> [u8; 16] {
        self.gilrs.borrow().gamepad(self.id).uuid()
    }

    pub fn is_force_feedback_supported(&self) -> bool {
        self.gilrs.borrow().gamepad(self.id).is_ff_supported()
    }

    pub fn power_info(&self) -> PowerInfo {
        self.gilrs.borrow().gamepad(self.id).power_info().into()
    }

    pub fn is_connected(&self) -> bool {
        self.gilrs.borrow().gamepad(self.id).is_connected()
    }

}

impl PartialEq for GamepadDevice {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}
