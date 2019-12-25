use super::Float;

#[derive(Debug, Copy, Clone)]
pub enum Angle<F: Float = f32> {
    Radians(F),
    Degrees(F),
}

impl<F: Float> Angle<F> {

    pub fn radians(value: F) -> Self {
        Angle::Radians(value)
    }

    pub fn degrees(value: F) -> Self {
        Angle::Degrees(value)
    }

    pub fn n_pi(n: F) -> Self {
        Self::radians(n * F::pi())
    }

    pub fn zero() -> Self {
        Self::radians(F::zero())
    }

    pub fn value(&self) -> F {
        match self {
            Angle::Radians(value) => *value,
            Angle::Degrees(value) => *value,
        }
    }

    pub fn radians_value(&self) -> F {
        match self {
            Angle::Radians(value) => *value,
            Angle::Degrees(value) => *value / F::straight_angle() * F::pi(),
        }
    }

    pub fn degrees_value(&self) -> F {
        match self {
            Angle::Radians(value) => *value / F::pi() * F::straight_angle(),
            Angle::Degrees(value) => *value,
        }
    }

    pub fn to_radians(&self) -> Self {
        Angle::Radians(self.radians_value())
    }

    pub fn to_degrees(&self) -> Self {
        Angle::Degrees(self.degrees_value())
    }

}

impl<F: Float> PartialEq for Angle<F> {

    fn eq(&self, other: &Self) -> bool {
        self.radians_value() == other.radians_value()
    }

}
