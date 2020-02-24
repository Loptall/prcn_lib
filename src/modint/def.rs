#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ModInt(pub u64);

use std::fmt;
impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 教訓: コンストラクタはなるだけ使うな
// -> 内部の値がMOD未満であることが保証されない
impl ModInt {
    /// ここの値、任意にするべきか
    pub const MOD: u64 = 1_000_000_007;

    /// コンストラクタ
    pub fn new(n: u64) -> ModInt {
        let mut ret = Self(n);
        ret.update();
        ret
    }

    /// 内部の値を一意に矯正する
    pub fn update(&mut self) {
        self.0 %= ModInt::MOD;
    }

    /// これはフィールドを公開するかしないかの問題で、
    /// 公開する場合はいらないんだけど内部の値が
    /// 外部から不正に操作されるのを防ぐために
    /// 非公開にするべきか迷ってる
    /// Rust風にいくならprivateにするべきか？
    pub fn to_int(self) -> u64 {
        self.0 as u64
    }
}

#[derive(Default)]
pub struct ComTable {
    fac: Vec<ModInt>,
    finv: Vec<ModInt>,
    inv: Vec<ModInt>,
}

impl ComTable {
    const MAX: usize = 510_000;

    pub fn new() -> Self {
        let mut ret = Self {
            fac: vec![ModInt::new(0); Self::MAX],
            finv: vec![ModInt::new(0); Self::MAX],
            inv: vec![ModInt::new(0); Self::MAX],
        };

        ret.fac[0] = ModInt::new(1);
        ret.fac[1] = ModInt::new(1);
        ret.finv[0] = ModInt::new(1);
        ret.finv[1] = ModInt::new(1);
        ret.inv[1] = ModInt::new(1);

        for i in 2..Self::MAX {
            ret.fac[i] = ModInt::new(ret.fac[i-1].0 * i as u64);
            ret.inv[i] = ModInt::new(ModInt::MOD - ret.inv[ModInt::MOD as usize % i].0 * (ModInt::new(ModInt::MOD).0 / ModInt::new(i as u64).0));
            ret.finv[i] = ret.finv[i-1] * ret.inv[i];
        }
        ret
    }

    pub fn combination(&self, n: u64, r: u64) -> ModInt {
        if n < r { ModInt(0) }
        else {
            self.fac[n as usize] * (self.finv[r as usize] * self.finv[(n - r) as usize])
        }
    }
}

