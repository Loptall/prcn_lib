#![cfg(test)]

use super::def::ModInt;
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

#[test]
fn fac_test() {
    assert_eq!(6, ModInt::new(3).factorial().0)
}