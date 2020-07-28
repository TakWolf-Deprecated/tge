
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum PowerInfo {
    Wired,
    Charged,
    Charging(u8),
    Discharging(u8),
    Unknown,
}

impl From<gilrs::PowerInfo> for PowerInfo {

    fn from(info: gilrs::PowerInfo) -> Self {
        match info {
            gilrs::PowerInfo::Wired => Self::Wired,
            gilrs::PowerInfo::Charged => Self::Charged,
            gilrs::PowerInfo::Charging(value) => Self::Charging(value),
            gilrs::PowerInfo::Discharging(value) => Self::Discharging(value),
            gilrs::PowerInfo::Unknown => Self::Unknown,
        }
    }

}
