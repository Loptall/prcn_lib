
use super::def::{ComTable, ModInt};

impl ModInt {
    /// ComTableの初期化が必要
    /// n < 510000
    /// 基本的な実装
    pub fn combination(t: &ComTable, n: u64, r: u64) -> ModInt {
        if n < r { ModInt(0) }
        else {
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