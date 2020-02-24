//! # More exceeded function for ModInt
//! - power
//! - combination
//! - other

use super::def::ModInt;

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

    /// ModInt上での二項係数を
    /// O(1)で返す
    ///
    /// rは通常の整数型で渡す
    pub fn combination(self, r: u64) -> Self {
        todo! {}
    }
}
