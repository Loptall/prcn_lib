
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ModInt(pub u64);

use std::fmt;
impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


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