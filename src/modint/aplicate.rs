//! # More exceeded function for ModInt
//! - power
//! - combination
//! - other

use super::def::{ComTable, ModInt};

use std::mem::swap;

impl ModInt {
    /// 二分累乗法,
    /// - `O(log n)`で累乗を求める
    pub fn pow_bin(self, n: u64) -> Self {
        let mut res = 1u64;
        let mut a = self;
        let mut n = n;
        while n > 0 {
            if n & 1 != 0 {
                res = res * a.to_int() % Self::MOD;
            }
            a *= a;
            n >>= 1;
        }
        Self::new(res)
    }

    #[allow(clippy::many_single_char_names)]
    pub fn inv(a: u64) -> Self {
        let mut a = a as i64;
        let mut b = Self::MOD as i64;
        let mut u = 1i64;
        let mut v = 0i64;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            swap(&mut a, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= Self::MOD as i64;
        if u < 0 {
            u += Self::MOD as i64;
        }
        Self(u as u64)
    }
}
