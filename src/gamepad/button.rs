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
            GamepadButton::LeftStick | GamepadButton::RightStick => true,
            _ => false,
        }
    }

    pub fn is_d_pad(&self) -> bool {
        match self {
            GamepadButton::DPadUp | GamepadButton::DPadDown | GamepadButton::DPadLeft | GamepadButton::DPadRight => true,
            _ => false,
        }
    }

    pub fn is_action(&self) -> bool {
        match self {
            GamepadButton::North | GamepadButton::South | GamepadButton::West | GamepadButton::East => true,
            _ => false,
        }
    }

    pub fn is_trigger(&self) -> bool {
        match self {
            GamepadButton::LeftTrigger | GamepadButton::LeftTrigger2 | GamepadButton::RightTrigger | GamepadButton::RightTrigger2 => true,
            _ => false,
        }
    }

    pub fn is_menu(&self) -> bool {
        match self {
            GamepadButton::Start | GamepadButton::Select | GamepadButton::Home => true,
            _ => false,
        }
    }

}

impl From<Button> for GamepadButton {

    fn from(button: Button) -> Self {
        match button {
            Button::South => GamepadButton::South,
            Button::East => GamepadButton::East,
            Button::North => GamepadButton::North,
            Button::West => GamepadButton::West,
            Button::C => GamepadButton::Other(1),
            Button::Z => GamepadButton::Other(2),
            Button::LeftTrigger => GamepadButton::LeftTrigger,
            Button::LeftTrigger2 => GamepadButton::LeftTrigger2,
            Button::RightTrigger => GamepadButton::RightTrigger,
            Button::RightTrigger2 => GamepadButton::RightTrigger2,
            Button::Select => GamepadButton::Select,
            Button::Start => GamepadButton::Start,
            Button::Mode => GamepadButton::Home,
            Button::LeftThumb => GamepadButton::LeftStick,
            Button::RightThumb => GamepadButton::RightStick,
            Button::DPadUp => GamepadButton::DPadUp,
            Button::DPadDown => GamepadButton::DPadDown,
            Button::DPadLeft => GamepadButton::DPadLeft,
            Button::DPadRight => GamepadButton::DPadRight,
            Button::Unknown => GamepadButton::Other(0),
        }
    }

}
