use std::fmt;
use num_traits::{Num, One, Zero, PrimInt};
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem,
    RemAssign, Neg
};
use nalgebra::{Field, SimdValue};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GF2(u8);

impl fmt::Display for GF2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let GF2(val) = self;
        write!(f, "{}", val)
    }
}

impl One for GF2 {
    fn one() -> Self { GF2(u8::one()) }
}

impl Zero for GF2 {
    fn zero() -> Self { GF2(u8::zero()) }

    fn is_zero(&self) -> bool {
        let GF2(val) = self;
        return val.is_zero();
    }
}

impl Add<Self> for GF2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let GF2(left) = self;
        let GF2(right) = rhs;
        GF2(left ^ right)
    }
}

impl AddAssign<Self> for GF2 {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Sub<Self> for GF2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { return self + rhs; }
}

impl SubAssign<Self> for GF2 {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Mul<Self> for GF2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let GF2(left) = self;
        let GF2(right) = rhs;
        GF2(left & right)
    }
}

impl MulAssign<Self> for GF2 {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl Div<Self> for GF2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Zero is an invalid divisor!");
        }
        self
    }
}

impl DivAssign<Self> for GF2 {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl Neg for GF2 {
    type Output = Self;

    fn neg(self) -> Self::Output { self }
}

impl Rem<Self> for GF2 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Zero is an invalid divisor!");
        }
        GF2::zero()
    }
}
impl RemAssign<Self> for GF2 {
    fn rem_assign(&mut self, rhs: Self) { *self = *self % rhs; }
}

impl SimdValue for GF2 {
    type Element = Self;
    type SimdBool = bool;

    fn lanes() -> usize { 1 }

    fn splat(val: Self::Element) -> Self { val }

    fn extract(&self, _: usize) -> Self::Element { *self }

    unsafe fn extract_unchecked(&self, _: usize) -> Self::Element { *self }

    fn replace(&mut self, _: usize, val: Self::Element) { *self = val; }

    unsafe fn replace_unchecked(&mut self, _: usize, val: Self::Element) { 
        *self = val;
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self { 
        if cond { self } else { other }
    }
}

impl Num for GF2 {
    type FromStrRadixErr = core::num::ParseIntError;

    fn from_str_radix(str: &str, radix: u32) 
        -> Result<Self, Self::FromStrRadixErr> {
        u8::from_str_radix(str, radix).map(|val| GF2(val % 2))
    }
}

impl Field for GF2 {}

impl<T> From<T> for GF2 where T: PrimInt {
    fn from(val: T) -> Self {
        let two = T::from(2).unwrap();
        let bin = (((val % two) + two) % two).to_u8().unwrap();
        GF2(bin)
    }
}