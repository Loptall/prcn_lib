use super::binomial_coefficient::BinomialCoefficient;
use super::permutation::permutation;
use crate::factorial::Factoriable;

use crate::modint::ModInt;

use cargo_snippet::snippet;

#[snippet(name = "combination", include = "binomial_coefficient")]
pub fn combination(n: ModInt, k: usize) -> ModInt {
    if k > n.get() as usize {
        panic!("n < k, where n in ModInt, k in usize, so cannot calculate n C k")
    }
    permutation(n, k) / k.factorial()
}

#[test]
fn comb_test() {
    use num_integer::binomial;
    let n = ModInt::new(4, 1000000007);
    let k = 2;
    assert_eq!(
        combination(n, k).get(),
        binomial(n.get() as usize, k) as i64
    );
    assert_eq!(combination(ModInt::new(10, 1000000007), 3).get(), 120);

    // let n = ModInt::new(1, 17);
    // let k = 3;
    // assert_eq!(combination(n, k).get(), 0); // panic!
}

#[snippet(name = "combination", include = "binomial_coefficient")]
pub fn combination_with_table<T: BinomialCoefficient>(table: &T, n: usize, k: usize) -> ModInt {
    table.binomial(n, k)
}

#[test]
fn comb_tbl_test() {
    use super::binomial_coefficient::{BCTSmallNK, BCTholdN, BCTDP};
    use num_integer::binomial;

    let tbl = BCTDP::new(10000, 1000000007);
    assert_eq!(combination_with_table(&tbl, 10, 3).get(), binomial(10, 3));
    assert_eq!(
        combination_with_table(&tbl, 100, 100).get(),
        binomial(100, 100)
    );
    assert_eq!(combination_with_table(&tbl, 2, 0).get(), binomial(2, 0));

    let tbl = BCTholdN::new(100, 1000000007);
    assert_eq!(combination_with_table(&tbl, 100, 3).get(), binomial(100, 3));
    assert_eq!(combination_with_table(&tbl, 100, 2).get(), binomial(100, 2));
    assert_eq!(combination_with_table(&tbl, 100, 0).get(), binomial(100, 0));

    let tbl = BCTSmallNK::new(1000, 1000000007);
    assert_eq!(combination_with_table(&tbl, 10, 3).get(), binomial(10, 3));
    assert_eq!(
        combination_with_table(&tbl, 100, 100).get(),
        binomial(100, 100)
    );
    assert_eq!(combination_with_table(&tbl, 2, 0).get(), binomial(2, 0));
}
