use super::binomial_coefficient::BCTDP;
use super::factorial::Factoriable;
use crate::modint::ModInt;

use cargo_snippet::snippet;

/// `n P k` を `O(k)` で
///
/// 内部はfallingをラップしているだけ
#[snippet("permutation")]
pub fn permutation<T: Factoriable>(n: T, k: usize) -> T {
    n.falling(k)
}

#[test]
fn permutation_test() {
    assert_eq!(permutation(10usize, 3), 10 * 9 * 8);
    assert_eq!(permutation(ModInt::new(6, 10), 4).get(), 6 * 5 * 4 * 3 % 10);
}

#[snippet("permutation")]
pub fn permutation_with_table(table: &BCTDP, n: usize, k: usize) -> ModInt {
    if k > n {
        ModInt::new(0, table.get_mod())
    } else {
        table.factorial(n) * table.factorial_inverse(n - k)
    }
}

#[test]
fn with_table_test() {
    let tbl = BCTDP::new(10, 1000000007);
    assert_eq!(permutation_with_table(&tbl, 4, 2).get(), 12);
    assert_eq!(permutation_with_table(&tbl, 10, 4).get(), 10 * 9 * 8 * 7);
    assert_eq!(permutation_with_table(&tbl, 1, 2).get(), 0);
}
