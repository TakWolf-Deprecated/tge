use super::Float;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::Ordering;

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

    pub fn set_value(&mut self, new_value: F) {
        match self {
            Angle::Radians(value) => *value = new_value,
            Angle::Degrees(value) => *value = new_value,
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

impl<F: Float> Add for Angle<F> {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Angle::Radians(value) => Angle::Radians(value + other.radians_value()),
            Angle::Degrees(value) => Angle::Degrees(value + other.degrees_value()),
        }
    }

}

impl<F: Float> Sub for Angle<F> {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Angle::Radians(value) => Angle::Radians(value - other.radians_value()),
            Angle::Degrees(value) => Angle::Degrees(value - other.degrees_value()),
        }
    }

}

impl<F: Float> AddAssign for Angle<F> {

    fn add_assign(&mut self, other: Self) {
        match self {
            Angle::Radians(value) => *value += other.radians_value(),
            Angle::Degrees(value) => *value += other.degrees_value(),
        }
    }

}

impl<F: Float> SubAssign for Angle<F> {

    fn sub_assign(&mut self, other: Self) {
        match self {
            Angle::Radians(value) => *value -= other.radians_value(),
            Angle::Degrees(value) => *value -= other.degrees_value(),
        }
    }

}

impl<F: Float> Mul<F> for Angle<F> {

    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        match self {
            Angle::Radians(value) => Angle::Radians(value * rhs),
            Angle::Degrees(value) => Angle::Degrees(value * rhs),
        }
    }

}

impl<F: Float> Div<F> for Angle<F> {

    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        match self {
            Angle::Radians(value) => Angle::Radians(value / rhs),
            Angle::Degrees(value) => Angle::Degrees(value / rhs),
        }
    }

}

impl<F: Float> MulAssign<F> for Angle<F> {

    fn mul_assign(&mut self, rhs: F) {
        match self {
            Angle::Radians(value) => *value *= rhs,
            Angle::Degrees(value) => *value *= rhs,
        }
    }

}

impl<F: Float> DivAssign<F> for Angle<F> {

    fn div_assign(&mut self, rhs: F) {
        match self {
            Angle::Radians(value) => *value /= rhs,
            Angle::Degrees(value) => *value /= rhs,
        }
    }

}

impl<F: Float> PartialEq for Angle<F> {

    fn eq(&self, other: &Self) -> bool {
        self.radians_value() == other.radians_value()
    }

}

impl<F: Float> PartialOrd for Angle<F> {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.radians_value().partial_cmp(&other.radians_value())
    }

}

#[cfg(test)]
mod tests {

    use super::Angle;

    #[test]
    fn create() {
        assert_eq!(Angle::<f32>::radians(std::f32::consts::PI * 2.0), Angle::<f32>::n_pi(2.0));
        assert_eq!(Angle::<f32>::n_pi(1.0), Angle::<f32>::degrees(180.0));
        assert_eq!(Angle::<f32>::n_pi(2.0).value(), std::f32::consts::PI * 2.0);
        assert_eq!(Angle::<f32>::degrees(90.0).value(), 90.0f32);
        assert_eq!(Angle::<f32>::zero(), Angle::<f32>::radians(0.0));
        assert_eq!(Angle::<f32>::zero(), Angle::<f32>::degrees(0.0));
        assert_eq!(Angle::<f32>::zero().value(), 0.0f32);
    }

    #[test]
    fn convert() {
        assert_eq!(Angle::<f32>::n_pi(1.0).to_degrees().value(), 180.0f32);
        assert_eq!(Angle::<f32>::degrees(90.0).to_radians().value(), std::f32::consts::PI * 0.5);
        assert_eq!(Angle::<f32>::n_pi(2.0).to_degrees().value(), Angle::<f32>::n_pi(2.0).degrees_value());
        assert_eq!(Angle::<f32>::degrees(180.0).to_radians().value(), Angle::<f32>::degrees(180.0).radians_value());
    }

    #[test]
    fn operator() {
        assert_eq!(Angle::<f32>::n_pi(1.0) + Angle::<f32>::n_pi(1.0), Angle::<f32>::n_pi(2.0));
        assert_eq!(Angle::<f32>::degrees(90.0) + Angle::<f32>::degrees(90.0), Angle::<f32>::degrees(180.0));
        assert_eq!(Angle::<f32>::n_pi(1.0) + Angle::<f32>::degrees(180.0), Angle::<f32>::n_pi(2.0));
        assert_eq!(Angle::<f32>::degrees(90.0) + Angle::<f32>::n_pi(0.5), Angle::<f32>::degrees(180.0));
        assert_eq!(Angle::<f32>::n_pi(2.0) - Angle::<f32>::n_pi(1.0), Angle::<f32>::n_pi(1.0));
        assert_eq!(Angle::<f32>::degrees(180.0) - Angle::<f32>::degrees(90.0), Angle::<f32>::degrees(90.0));
        assert_eq!(Angle::<f32>::n_pi(2.0) - Angle::<f32>::degrees(180.0), Angle::<f32>::n_pi(1.0));
        assert_eq!(Angle::<f32>::degrees(180.0) - Angle::<f32>::n_pi(0.5), Angle::<f32>::degrees(90.0));
        assert_eq!(Angle::<f32>::n_pi(1.0) * 2.0, Angle::<f32>::n_pi(2.0));
        assert_eq!(Angle::<f32>::degrees(90.0) * 2.0, Angle::<f32>::degrees(180.0));
        assert_eq!(Angle::<f32>::n_pi(2.0) / 2.0, Angle::<f32>::n_pi(1.0));
        assert_eq!(Angle::<f32>::degrees(90.0) / 2.0, Angle::<f32>::degrees(45.0));

        let mut angle = Angle::<f32>::n_pi(2.0);
        angle += Angle::<f32>::n_pi(2.0);
        assert_eq!(angle, Angle::<f32>::n_pi(4.0));
        angle += Angle::<f32>::degrees(180.0);
        assert_eq!(angle, Angle::<f32>::n_pi(5.0));
        angle -= Angle::<f32>::n_pi(1.0);
        assert_eq!(angle, Angle::<f32>::n_pi(4.0));
        angle -= Angle::<f32>::degrees(180.0);
        assert_eq!(angle, Angle::<f32>::n_pi(3.0));

        let mut angle = Angle::<f32>::degrees(180.0);
        angle += Angle::<f32>::degrees(180.0);
        assert_eq!(angle, Angle::<f32>::degrees(360.0));
        angle += Angle::<f32>::n_pi(2.0);
        assert_eq!(angle, Angle::<f32>::degrees(720.0));
        angle -= Angle::<f32>::degrees(360.0);
        assert_eq!(angle, Angle::<f32>::degrees(360.0));
        angle -= Angle::<f32>::n_pi(1.0);
        assert_eq!(angle, Angle::<f32>::degrees(180.0));

        let mut angle = Angle::<f32>::n_pi(1.0);
        angle *= 4.0;
        assert_eq!(angle, Angle::<f32>::n_pi(4.0));
        angle /= 2.0;
        assert_eq!(angle, Angle::<f32>::n_pi(2.0));

        let mut angle = Angle::<f32>::degrees(180.0);
        angle *= 4.0;
        assert_eq!(angle, Angle::<f32>::degrees(720.0));
        angle /= 2.0;
        assert_eq!(angle, Angle::<f32>::degrees(360.0));
    }

    #[test]
    fn compare() {
        assert!(Angle::<f32>::n_pi(1.5) > Angle::<f32>::n_pi(0.5));
        assert!(Angle::<f32>::n_pi(1.0) < Angle::<f32>::n_pi(2.0));
        assert!(Angle::<f32>::degrees(90.0) > Angle::<f32>::degrees(45.0));
        assert!(Angle::<f32>::degrees(30.0) < Angle::<f32>::degrees(60.0));
        assert!(Angle::<f32>::n_pi(2.0) > Angle::<f32>::degrees(300.0));
        assert!(Angle::<f32>::n_pi(0.5) < Angle::<f32>::degrees(135.0));
    }

}
