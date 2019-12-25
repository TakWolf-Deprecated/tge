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
