use super::Number;

pub trait Float: Number {
    fn pi() -> Self;

    fn straight_angle() -> Self;
}

impl Float for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn straight_angle() -> Self {
        180.0
    }
}

impl Float for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn straight_angle() -> Self {
        180.0
    }
}
