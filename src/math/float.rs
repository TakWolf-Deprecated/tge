use super::Number;

pub trait Float: Number {
    fn pi() -> Self;

    fn to_degrees(self) -> Self;

    fn to_radians(self) -> Self;
}

impl Float for f32 {
    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    fn to_radians(self) -> Self {
        self.to_radians()
    }
}

impl Float for f64 {
    fn pi() -> Self {
        std::f64::consts::PI
    }

    fn to_degrees(self) -> Self {
        self.to_degrees()
    }

    fn to_radians(self) -> Self {
        self.to_radians()
    }
}
