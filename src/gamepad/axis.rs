use gilrs::Axis;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    LeftZ,
    RightStickX,
    RightStickY,
    RightZ,
    DPadX,
    DPadY,
    Other(u32),
}

impl From<Axis> for GamepadAxis {

    fn from(axis: Axis) -> Self {
        match axis {
            Axis::LeftStickX => GamepadAxis::LeftStickX,
            Axis::LeftStickY => GamepadAxis::LeftStickY,
            Axis::LeftZ => GamepadAxis::LeftZ,
            Axis::RightStickX => GamepadAxis::RightStickX,
            Axis::RightStickY => GamepadAxis::RightStickY,
            Axis::RightZ => GamepadAxis::RightZ,
            Axis::DPadX => GamepadAxis::DPadX,
            Axis::DPadY => GamepadAxis::DPadY,
            Axis::Unknown => GamepadAxis::Other(0),
        }
    }

}
