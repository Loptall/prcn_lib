// # Generic
// use std::ops::{Add, Div, Mul, Rem};
// /// 試し割りで整数`n`を素数判定する。
// ///
// /// O(√n)
// pub fn is_prime<T>(n: T) -> bool
// where
//     T: Div<Output = T>
//         + Rem<Output = T>
//         + Mul<Output = T>
//         + Add<Output = T>
//         + Copy
//         + From<u8>
//         + PartialOrd,
// {
//     if n <= T::from(1) {
//         false
//     } else if n == T::from(2) {
//         true
//     } else if n % T::from(2) == T::from(0) {
//         false
//     } else {
//         let mut d = T::from(3);
//         loop {
//             if d * d > n {
//                 break true;
//             }
//             if n % d == T::from(0) {
//                 break false;
//             }
//             d = d + T::from(2);
//         }
//     }
// }

use cargo_snippet::snippet;

/// 2以上で`x`の約数であるものの内最小であるものを求める
#[snippet("trial_division")]
fn firstfac(x: usize) -> usize {
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

/// 試し割りによる素数判定です
///
/// `O(√n)`
#[snippet("trial_division")]
pub fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}

#[test]
fn is_prime_test() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(17), true);
    assert_eq!(is_prime(1000000007), true);
}

#[snippet("trial_division")]
fn exp(n: &mut usize, d: usize) -> (usize, usize) {
    let mut c = 0usize;
    (
        d,
        loop {
            if *n % d != 0 {
                break c;
            }
            *n /= d;
            c += 1;
        },
    )
}

#[snippet("trial_division")]
pub fn factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut primes = Vec::new();
    let (d, e) = exp(&mut n, 2);
    if e > 0 {
        primes.push((d, e));
    }
    let m = n;
    for i in (3..).take_while(|x| *x * *x <= m).step_by(2) {
        let (d, e) = exp(&mut n, i);
        if e > 0 {
            primes.push((d, e));
        }
    }
    if n > 1 {
        match primes.iter().enumerate().find(|x| (x.1).0 == n) {
            Some((i, _)) => primes[i].1 += 1,
            None => primes.push((n, 1)),
        }
    }
    primes
}

#[test]
fn f_test() {
    let f = factorization(13);
    assert_eq!(f, vec![(13, 1)]);
    let f = factorization(12);
    assert_eq!(f, vec![(2, 2), (3, 1)]);
    let f = factorization(1000000007);
    assert_eq!(f, vec![(1000000007, 1)]);
}
