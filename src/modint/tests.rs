#![cfg(test)]

use super::def::{ModInt, ComTable};
use rand::random;

#[test]
fn add() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!((a + b).0, ((a.0 as u128 + b.0 as u128) % (ModInt::MOD as u128)) as u64)
    }
}

#[test]
fn sub() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!((a - b).0, if ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) >= 0 {
            ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) as u64
        } else {
            (((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) + ModInt::MOD as i128) as u64
        })
    }
}

#[test]
fn mul() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!((a * b).0, ((a.0 as u128 * b.0 as u128) % (ModInt::MOD as u128)) as u64)
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


// #[test]
// fn pow() {
//     for _i in 0..10000 {
//         let a = loop {
//             match random::<u64>() {
//                 n if n < ModInt::MOD => break n,
//                 _ => continue,
//             }
//         };
//         let b = loop {
//             match random::<u64>() {
//                 n if n < ModInt::MOD => break n,
//                 _ => continue,
//             }
//         };
//         let mut ans = 1;
//         for _i in 0..b as usize {
//             ans *= a;
//             ans %= ModInt::MOD;
//         }
//         assert_eq!(ModInt::new(a).pow_bin(b).0, ans)
//     }
// }


#[test]
fn com() {
    let table = ComTable::new();
    for _i in 0..10000 {
        let a = random::<u16>() as u64;
        let b = random::<u16>() as u64;
        let ans = factorial(a) / (factorial(a - b) * factorial(b) % ModInt::MOD);

        assert_eq!(table.combination(a, b), ModInt::new(ans))
    }
}

fn factorial(n: u64) -> u64 {
    let mut ret = 1;
    for i in 2..=n {
        ret *= i;
        ret %= ModInt::MOD;
    }
    ret
}