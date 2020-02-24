use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ModInt(i64);

impl fmt::Display for ModInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ModInt.0 が常に非負であることを保証したい。
// update()にその役目を持たせて定期的に更新するのを忘れないようにする。
// 少なくともインターフェイスから見える部分では

impl ModInt {
    pub const MOD: i64 = 1_000_000_007;

    /// コンストラクタ
    pub fn new(n: i64) -> ModInt {
        // ここではインターフェイスに密着した場所なのでupdate
        // ではなく手動でハンドル
        ModInt(if n >= 0 {
            n % ModInt::MOD
        } else {
            (n % ModInt::MOD + ModInt::MOD)
        })
    }

    /// 内部の値を一意に矯正する
    fn update(&mut self) {
        self.0 = if self.0 >= 0 {
            self.0 % ModInt::MOD
        } else {
            (self.0 % ModInt::MOD + ModInt::MOD)
        }
    }

    pub fn to_int(self) -> i64 {
        self.0 as i64
    }
}

// diriveでけたやん...

// impl PartialEq for ModInt {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0
//     }
// }
// impl Eq for ModInt {}

// 以下演算
// selfとotherは常に非負と思って良い

impl Add for ModInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % ModInt::MOD)
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

// subだけは負の数が現れることもある

impl Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut ret = Self((self.0 - other.0) % ModInt::MOD);
        ret.update();
        ret
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.update();
    }
}

impl Mul for ModInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self((self.0 * other.0) % ModInt::MOD)
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.update();
    }
}

// Fermatの小定理

impl Div for ModInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        todo! {}
    }
}

impl DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        todo! {}
    }
}
