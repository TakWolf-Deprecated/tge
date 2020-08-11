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
            Axis::LeftStickX => Self::LeftStickX,
            Axis::LeftStickY => Self::LeftStickY,
            Axis::LeftZ => Self::LeftZ,
            Axis::RightStickX => Self::RightStickX,
            Axis::RightStickY => Self::RightStickY,
            Axis::RightZ => Self::RightZ,
            Axis::DPadX => Self::DPadX,
            Axis::DPadY => Self::DPadY,
            Axis::Unknown => Self::Other(0),
        }
    }
}
