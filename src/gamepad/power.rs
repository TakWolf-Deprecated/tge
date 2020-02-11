
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
            gilrs::PowerInfo::Wired => PowerInfo::Wired,
            gilrs::PowerInfo::Charged => PowerInfo::Charged,
            gilrs::PowerInfo::Charging(value) => PowerInfo::Charging(value),
            gilrs::PowerInfo::Discharging(value) => PowerInfo::Discharging(value),
            gilrs::PowerInfo::Unknown => PowerInfo::Unknown,
        }
    }

}
