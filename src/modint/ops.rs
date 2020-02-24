#![allow(clippy::suspicious_arithmetic_impl)]

use super::def::ModInt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// 以下演算
// selfとotherは常に非負と思って良い

impl Add for ModInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % ModInt::MOD)
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

// subだけは負の数が現れることもあるので調整
use std::cmp::Ordering;
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut ret = match (self.0).cmp(&other.0) {
            Ordering::Less => Self(self.0 + Self::MOD - other.0),
            _ => Self(self.0 - other.0),
        };
        ret.update();
        ret
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self((self.0 * other.0) % ModInt::MOD)
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

// 要履修: "Fermatの小定理"

impl Div for ModInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let inv = Self::inv(other.0);
        self * inv
    }
}

impl DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
