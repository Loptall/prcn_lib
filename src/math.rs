//! 

use num::{Integer, zero, one};
use std::convert::{Into, From};
use std::ops::{DivAssign, Mul, MulAssign, AddAssign};
// use num::BigInt;

/// `n`を十進法で表したときの桁数
pub fn scale_dight<T: Integer + DivAssign + Mul + From<u8>>(n: T) -> usize {
    if n == zero() {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= one() {
        n /= one::<T>() * 10u8.into();
        count += 1;
    }
    count
}

#[test]
fn test_scale() {
    let a = 99i64;
    assert!(2 == scale_dight(a));
    let b = 0usize;
    assert!(1 == scale_dight(b));
    let c: num::BigInt = num::BigInt::from(21_746_284_928_973_i128);
    assert!(14 == scale_dight(c));
}

/// `n`を`base`進法で表したときの桁数
pub fn scale_n_base<T: Integer + DivAssign + Mul + From<u8>>(n: T, base: u8) -> usize {
    if n == zero() {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= one() {
        n /= one::<T>() * base.into();
        count += 1;
    }
    count
}

#[test]
fn test_n_base_scale() {
    let a = 99i64;
    assert_eq!(7, scale_n_base(a, 2));
    let b = 0usize;
    assert_eq!(1, scale_n_base(b, 100));
    let c: num::BigInt = num::BigInt::from(21_746_284_928_973_i128) * num::BigInt::from(11_111_111_111_111_111_i128);
    assert_eq!(35, scale_n_base(c, 7));
}

/// 整数を桁ごとに`Vec<u64>`に分解
pub fn dight_vec<T: std::ops::MulAssign + std::convert::From<u8> + std::ops::DivAssign + Integer + Copy>(n: T) -> Vec<T> {
    let mut idx = scale_dight(n) - 1;
    let mut ret = Vec::new();
    loop {
        ret.push(n / pow_bin(10.into(), idx as u32) % 10.into());
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    ret
}

#[test]
fn vec_test() {
    let a = dight_vec(12345usize);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}

/// 整数の十進法での各桁の和
pub fn dight_sum<T: std::ops::MulAssign + std::convert::From<u8> + std::ops::DivAssign + Integer + Copy + AddAssign>(n: T) -> T {
    let mut res = zero();
    for i in dight_vec(n) {
        res += i;
    }
    res
}

#[test]
fn sum_test() {
    let n = 1234;
    assert_eq!(10, dight_sum(n));
}

/// 二分累乗法,
/// - `O(log n)`で累乗を求める
pub fn pow_bin<T: Integer + MulAssign + Copy>(n: T, r: u32) -> T {
    let mut res: T = one();
    let mut a = n;
    let mut n = r;
    while n > 0 {
        if n & 1 != 0 {
            res *= a
        }
        a *= a;
        n >>= 1;
    }
    res
}

// TODO
// - move to prime module

/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };

    for n in (3..).step_by(2).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }

    x
}

/// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}
