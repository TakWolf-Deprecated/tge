use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait Number: Sized + Copy + Clone + PartialEq + PartialOrd + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + AddAssign + SubAssign + MulAssign + DivAssign {

    fn zero() -> Self;

}

impl Number for u8 {

    fn zero() -> Self {
        0
    }

}

impl Number for i8 {

    fn zero() -> Self {
        0
    }

}

impl Number for u16 {

    fn zero() -> Self {
        0
    }

}

impl Number for i16 {

    fn zero() -> Self {
        0
    }

}

impl Number for u32 {

    fn zero() -> Self {
        0
    }

}

impl Number for i32 {

    fn zero() -> Self {
        0
    }

}

impl Number for u64 {

    fn zero() -> Self {
        0
    }

}

impl Number for i64 {

    fn zero() -> Self {
        0
    }

}

impl Number for u128 {

    fn zero() -> Self {
        0
    }

}

impl Number for i128 {

    fn zero() -> Self {
        0
    }

}

impl Number for usize {

    fn zero() -> Self {
        0
    }

}

impl Number for isize {

    fn zero() -> Self {
        0
    }

}

impl Number for f32 {

    fn zero() -> Self {
        0.0
    }

}

impl Number for f64 {

    fn zero() -> Self {
        0.0
    }

}

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
