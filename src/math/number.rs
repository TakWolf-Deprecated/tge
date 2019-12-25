use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait Number: Copy + Clone + Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + AddAssign + SubAssign + MulAssign + DivAssign {

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
