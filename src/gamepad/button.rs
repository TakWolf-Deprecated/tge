use gilrs::Button;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum GamepadButton {
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    North,
    South,
    West,
    East,
    LeftTrigger,
    LeftTrigger2,
    RightTrigger,
    RightTrigger2,
    Start,
    Select,
    Home,
    Other(u32),
}

impl GamepadButton {
    pub fn is_stick(&self) -> bool {
        match self {
            Self::LeftStick | Self::RightStick => true,
            _ => false,
        }
    }

    pub fn is_d_pad(&self) -> bool {
        match self {
            Self::DPadUp | Self::DPadDown | Self::DPadLeft | Self::DPadRight => true,
            _ => false,
        }
    }

    pub fn is_action(&self) -> bool {
        match self {
            Self::North | Self::South | Self::West | Self::East => true,
            _ => false,
        }
    }

    pub fn is_trigger(&self) -> bool {
        match self {
            Self::LeftTrigger | Self::LeftTrigger2 | Self::RightTrigger | Self::RightTrigger2 => true,
            _ => false,
        }
    }

    pub fn is_menu(&self) -> bool {
        match self {
            Self::Start | Self::Select | Self::Home => true,
            _ => false,
        }
    }
}

impl From<Button> for GamepadButton {
    fn from(button: Button) -> Self {
        match button {
            Button::South => Self::South,
            Button::East => Self::East,
            Button::North => Self::North,
            Button::West => Self::West,
            Button::C => Self::Other(1),
            Button::Z => Self::Other(2),
            Button::LeftTrigger => Self::LeftTrigger,
            Button::LeftTrigger2 => Self::LeftTrigger2,
            Button::RightTrigger => Self::RightTrigger,
            Button::RightTrigger2 => Self::RightTrigger2,
            Button::Select => Self::Select,
            Button::Start => Self::Start,
            Button::Mode => Self::Home,
            Button::LeftThumb => Self::LeftStick,
            Button::RightThumb => Self::RightStick,
            Button::DPadUp => Self::DPadUp,
            Button::DPadDown => Self::DPadDown,
            Button::DPadLeft => Self::DPadLeft,
            Button::DPadRight => Self::DPadRight,
            Button::Unknown => Self::Other(0),
        }
    }
}
