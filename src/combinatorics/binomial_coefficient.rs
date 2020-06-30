use num_traits::Zero;

use crate::modint::{IntoModInt, ModInt};

use std::num::NonZeroU32;

use cargo_snippet::snippet;

#[snippet("binomial_coefficient", include = "modint")]
pub trait PartialBinomialCoefficient {
    fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt>;
}

#[snippet("binomial_coefficient")]
pub trait BinomialCoefficient: PartialBinomialCoefficient {
    /// `n C k`
    fn binomial(&self, n: usize, k: usize) -> ModInt {
        self.partial_binomial(n, k).unwrap()
    }
}

/// Binomial Coefficient Table with DP
/// 二項係数を`O(1)`で計算するためのテーブル
///
/// factrial = [1, 1, 2, 6, 24, 120, ...],
///
/// `1 <= k <= n <= 10^7` 程度
#[snippet("binomial_coefficient")]
pub struct BCTDP {
    _modulo: NonZeroU32,
    // `factorial[i]` = iの階乗
    factorial: Vec<ModInt>,
    // `inv[i]` = iの逆元
    inverse: Vec<ModInt>,
    // `factorial_inverse[i]` = iの階乗の逆元
    factorial_inverse: Vec<ModInt>,
}

#[snippet("binomial_coefficient")]
impl BCTDP {
    /// 初期化
    ///
    /// DPを用いて `O(n log m)`
    /// 割り算を用いるので `log m` がつく
    ///
    /// `1 <= k <= n <= 10^7` 程度
    pub fn new(n: usize, modulo: usize) -> BCTDP {
        let mut factorial = vec![ModInt::new(1, modulo), ModInt::new(1, modulo)];
        factorial.reserve_exact(n);
        let mut inverse = vec![ModInt::new(0, modulo), ModInt::new(1, modulo)];
        inverse.reserve_exact(n);
        let mut factorial_inverse = vec![ModInt::new(1, modulo), ModInt::new(1, modulo)];
        factorial_inverse.reserve_exact(n);

        for i in 2..=n {
            factorial.push(factorial[i - 1] * i);
            inverse.push(modulo.to_mint(modulo) - inverse[modulo % i] * (modulo / i));
            factorial_inverse.push(factorial_inverse[i - 1] * inverse[i]);
        }

        Self {
            _modulo: NonZeroU32::new(modulo as u32).unwrap(),
            factorial,
            inverse,
            factorial_inverse,
        }
    }

    pub fn get_mod(&self) -> usize {
        self._modulo.get() as usize
    }

    pub fn factorial(&self, n: usize) -> ModInt {
        self.factorial[n]
    }

    pub fn factorial_inverse(&self, n: usize) -> ModInt {
        self.factorial_inverse[n]
    }

    /// `n` の mod self._modulo における逆元
    pub fn inv(&self, n: usize) -> ModInt {
        self.inverse[n]
    }
}

#[snippet("binomial_coefficient")]
impl PartialBinomialCoefficient for BCTDP {
    fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt> {
        Some(if n < k {
            ModInt::zero()
        } else {
            self.factorial[n] * self.factorial_inverse[k] * self.factorial_inverse[n - k]
        })
    }
}

#[snippet("binomial_coefficient")]
impl BinomialCoefficient for BCTDP {}

#[test]
fn binomial_dp() {
    let tbl = BCTDP::new(10, 1000000007);
    assert_eq!(tbl.binomial(3, 2).get(), 3);
    assert_eq!(tbl.binomial(5, 2).get(), 10);
    assert_eq!(tbl.binomial(10, 6).get(), 210);
}

#[test]
fn bct_api_test() {
    let tbl = BCTDP::new(100, 1000000007);
    assert_eq!(tbl.factorial(5).get(), 120);
    assert_eq!(tbl.factorial(8).get(), 120 * 6 * 7 * 8);
}

/// `n` が固定値のときに有効
/// `(n(固定値), mod, _[i] = n C i)`
///
/// 初期化: `O(n)`
///
/// `1 <= n <= 10^9 && 1 <= k <= 10^7` 程度
#[snippet("binomial_coefficient")]
pub struct BCTholdN(usize, NonZeroU32, Vec<ModInt>);

#[snippet("binomial_coefficient")]
impl BCTholdN {
    pub fn new(mut n: usize, m: usize) -> Self {
        let size = n;
        let mut c = vec![ModInt::new(1, m), ModInt::new(n, m)];
        c.reserve_exact(n + 1);
        for i in 2..=n {
            n -= 1;
            let prev = *c.last().unwrap();
            c.push(prev * n / i);
        }

        Self(size, NonZeroU32::new(m as u32).unwrap(), c)
    }
}

#[snippet("binomial_coefficient")]
impl PartialBinomialCoefficient for BCTholdN {
    /// #Panic
    ///
    /// self.0 == _n でないとき
    fn partial_binomial(&self, _n: usize, k: usize) -> Option<ModInt> {
        if _n != self.0 {
            None
        } else {
            Some(self.2[k])
        }
    }
}

impl BinomialCoefficient for BCTholdN {}

#[test]
fn hold_n_test() {
    let tbl = BCTholdN::new(10, 1000000007);
    assert_eq!(tbl.partial_binomial(10, 2).unwrap().get(), 45);
    assert_eq!(tbl.partial_binomial(10, 10).unwrap().get(), 1);
}

/// `n, k` の2変数についての `n C k` の表を作る
///
/// `1 <= k <= n <= 2000` 程度
#[snippet("binomial_coefficient")]
pub struct BCTSmallNK {
    n: usize,
    _modulo: NonZeroU32,
    dp: Vec<Vec<ModInt>>,
}

#[snippet("binomial_coefficient")]
impl BCTSmallNK {
    pub fn new(n: usize, modulo: usize) -> Self {
        let mut dp = vec![vec![ModInt::new(0, modulo); n + 1]; n + 1];
        dp[0][0] = 1.to_mint(modulo);
        for i in 1..n {
            dp[i][0] = 1.to_mint(modulo);
            for j in 1..n {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }
        Self {
            n,
            _modulo: NonZeroU32::new(modulo as u32).unwrap(),
            dp,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get_mod(&self) -> usize {
        self._modulo.get() as usize
    }
}

#[snippet("binomial_coefficient")]
impl PartialBinomialCoefficient for BCTSmallNK {
    fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt> {
        if n > self.size() || k > self.size() {
            panic!("n or k is too large, compere to dp table!")
        }
        Some(self.dp[n][k])
    }
}

#[snippet("binomial_coefficient")]
impl BinomialCoefficient for BCTSmallNK {}

#[test]
fn small_test() {
    let tbl = BCTSmallNK::new(100, 1000000007);
    assert_eq!(tbl.binomial(4, 2).get(), 6);
}
