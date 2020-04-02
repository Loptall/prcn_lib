//! Varified
//!
//! TODO
//! - apply num-traits to Modint

#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(unused_imports)]
use rand::random;
use std::mem::swap;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ModInt(pub u64);

use std::fmt;
impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 教訓: コンストラクタはなるだけ使うな
// -> 内部の値がMOD未満であることが保証されない
impl ModInt {
    /// ここの値、任意にするべきか
    pub const MOD: u64 = 1_000_000_007;

    /// コンストラクタ
    pub fn new(n: u64) -> ModInt {
        let mut ret = Self(n);
        ret.update();
        ret
    }

    /// 内部の値を一意に矯正する
    pub fn update(&mut self) {
        self.0 %= ModInt::MOD;
    }
}

impl ModInt {
    /// 二分累乗法,
    /// - `O(log n)`で累乗を求める
    pub fn pow_bin(self, n: u64) -> Self {
        let mut res = 1u64;
        let mut a = self;
        let mut n = n;
        while n > 0 {
            if n & 1 != 0 {
                res = res * a.0 % Self::MOD;
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

        impl ModInt {
            /// ComTableの初期化が必要
            /// n < 510000
            /// 基本的な実装
            pub fn combination(t: &ComTable, n: u64, r: u64) -> ModInt {
                if n < r {
                    ModInt(0)
                } else {
                    t.fac[n as usize] * t.finv[r as usize] * t.finv[(n - r) as usize]
                }
            }

            /// nが大きく
            /// rは小さい時
            pub fn comb_big(n: ModInt, r: ModInt) -> ModInt {
                let mut ans = ModInt::new(1);
                for i in n.0 - r.0 + 1..=n.0 {
                    ans *= ModInt::new(i);
                }
                for i in 1..=r.0 {
                    ans /= ModInt::new(i);
                }
                ans
            }
        }

        if u < 0 {
            u += Self::MOD as i64;
        }
        Self(u as u64)
    }

    pub fn factorial(self) -> Self {
        let mut ret = ModInt::new(1);
        for i in 2..=self.0 as usize {
            ret *= ModInt::new(i as u64);
        }
        ret
    }
}

#[derive(Default)]
pub struct ComTable {
    pub fac: Vec<ModInt>,
    pub finv: Vec<ModInt>,
}

impl ComTable {
    const MAX: usize = 510_000;

    pub fn new() -> Self {
        let mut ret = Self {
            fac: vec![ModInt::new(0); Self::MAX],
            finv: vec![ModInt::new(0); Self::MAX],
        };

        ret.fac[0] = ModInt::new(1);
        ret.fac[1] = ModInt::new(1);

        for i in 2..Self::MAX {
            ret.fac[i] = ModInt::new(ret.fac[i - 1].0 * i as u64);
        }

        ret.finv[ComTable::MAX - 1] = ret.fac[ComTable::MAX - 1].pow_bin(ModInt::MOD - 2);
        for i in (1..Self::MAX).rev() {
            ret.finv[i - 1] = ret.finv[i] * ModInt::new(i as u64);
        }
        ret
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

#[test]
fn add() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a + b).0,
            ((a.0 as u128 + b.0 as u128) % (ModInt::MOD as u128)) as u64
        )
    }
}

#[test]
fn sub() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a - b).0,
            if ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) >= 0 {
                ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) as u64
            } else {
                (((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) + ModInt::MOD as i128) as u64
            }
        )
    }
}

#[test]
fn mul() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a * b).0,
            ((a.0 as u128 * b.0 as u128) % (ModInt::MOD as u128)) as u64
        )
    }
}

#[test]
fn div() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!((a / b).0 * b.0 % ModInt::MOD, a.0)
    }
}

#[test]
fn fac_test() {
    assert_eq!(6, ModInt::new(3).factorial().0)
}
