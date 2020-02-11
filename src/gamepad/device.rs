use gilrs::{Gilrs, Gamepad};
use std::rc::Rc;
use std::cell::RefCell;

pub type GamepadId = gilrs::GamepadId;

#[derive(Debug, Clone)]
pub struct GamepadDevice {
    gilrs: Rc<RefCell<Gilrs>>,
    id: GamepadId,
}

impl GamepadDevice {

    pub(crate) fn new(gilrs: Rc<RefCell<Gilrs>>, id: GamepadId) -> Self {
        Self { gilrs, id }
    }

    fn gamepad(&self) -> Gamepad {
        self.gilrs.borrow().gamepad(self.id)
    }

    pub fn id(&self) -> GamepadId {
        self.id
    }

    pub fn name(&self) -> &str {
        self.gamepad().name()
    }

    pub fn uuid(&self) -> [u8; 16] {
        self.gamepad().uuid()
    }

    pub fn is_force_feedback_supported(&self) -> bool {
        self.gamepad().is_ff_supported()
    }

    pub fn is_connected(&self) -> bool {
        self.gamepad().is_connected()
    }

}

impl PartialEq for GamepadDevice {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}
