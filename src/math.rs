//! 数字、計算に関するメソッドを取り扱う

use std::cmp::{max, min};

/// `u64`の十進法での桁数
pub fn dight_scale(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= 1 {
        n /= 10;
        count += 1;
    }
    count
}

/// `u64`を桁ごとに`Vec<u64>`に分解
pub fn dight_vec(n: u64) -> Vec<u64> {
    let mut idx: u64 = dight_scale(n) - 1;
    let mut ret = Vec::new();
    loop {
        ret.push(n / pow_bin(10, idx) % 10u64);
        if idx == 0u64 {
            break;
        }
        idx -= 1;
    }
    ret
}

/// `u64`の十進法での各桁の和
pub fn dight_sum(n: u64) -> u64 {
    dight_vec(n).iter().sum()
}

/// 二分累乗法,
/// - `O(log n)`で累乗を求める
pub fn pow_bin(n: u64, r: u64) -> u64 {
    let mut res = 1u64;
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

// /// 互除法を用いて最大公約数を求める
// pub fn gcd(n: u64, m: u64) -> u64 {
//     let (mut n, mut m) = (max(n, m), min(n, m));
//     if m == 0 {
//         return n;
//     }
//     let mut _r = 0;
//     while n % m != 0 {
//         _r = n % m;
//         n = m;
//         m = _r;
//     }
//     m
// }

// /// 最小公倍数を求める
// pub fn lcm(n: u64, m: u64) -> u64 {
//     n * m / gcd(n, m)
// }

// /// 配列全体の最大公約数
// pub fn gcd_vec(v: &[u64]) -> u64 {
//     let mut r = v[0];
//     for i in v.iter().skip(1) {
//         r = gcd(r, *i);
//     }
//     r
// }

// /// 配列全体の最小公倍数
// pub fn lcm_vec(v: &[u64]) -> u64 {
//     let mut r = v[0];
//     for i in v.iter().skip(1) {
//         r = lcm(r, *i);
//     }
//     r
// }

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
