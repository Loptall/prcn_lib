// ライブラリの内容を1ファイルに詰め込んで単体で提出できるようにした。
// ライブラリ置き場 -> https://crates.io/crates/prcn_lib

// from std
pub use std::cmp::{max, min};
pub use std::collections::{HashMap, HashSet, VecDeque};

// from proconio
pub use proconio::{fastout, input, marker::*};

pub use rand::random;

// use rand::random;
use std::mem::swap;

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

    #[allow(clippy::many_single_char_names)]
    pub fn inv(a: u64) -> Self {
        let mut a = a as i64;
        let mut b = Self::MOD as i64;
        let mut u = 1i64;
        let mut v = 0i64;
        while b != 0 {
            let t = a / b;
            a -= t * b;
            swap(&mut a, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= Self::MOD as i64;

        impl ModInt {
            /// ComTableの初期化が必要
            /// n < 510000
            /// 基本的な実装
            pub fn combination(t: &ComTable, n: u64, r: u64) -> ModInt {
                if n < r {
                    ModInt(0)
                } else {
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

        if u < 0 {
            u += Self::MOD as i64;
        }
        Self(u as u64)
    }

    pub fn factorial(self) -> Self {
        let mut ret = ModInt::new(1);
        for i in 2..=self.0 as usize {
            ret *= ModInt::new(i as u64);
        }
        ret
    }
}

#[derive(Default)]
pub struct ComTable {
    pub fac: Vec<ModInt>,
    pub finv: Vec<ModInt>,
}

impl ComTable {
    const MAX: usize = 510_000;

    pub fn new() -> Self {
        let mut ret = Self {
            fac: vec![ModInt::new(0); Self::MAX],
            finv: vec![ModInt::new(0); Self::MAX],
        };

        ret.fac[0] = ModInt::new(1);
        ret.fac[1] = ModInt::new(1);

        for i in 2..Self::MAX {
            ret.fac[i] = ModInt::new(ret.fac[i - 1].0 * i as u64);
        }

        ret.finv[ComTable::MAX - 1] = ret.fac[ComTable::MAX - 1].pow_bin(ModInt::MOD - 2);
        for i in (1..Self::MAX).rev() {
            ret.finv[i - 1] = ret.finv[i] * ModInt::new(i as u64);
        }
        ret
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
        *self = *self + other;
    }
}

// subだけは負の数が現れることもあるので調整
use std::cmp::Ordering;
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut ret = match (self.0).cmp(&other.0) {
            Ordering::Less => Self(self.0 + Self::MOD - other.0),
            _ => Self(self.0 - other.0),
        };
        ret.update();
        ret
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
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
        *self = *self * other;
    }
}

// 要履修: "Fermatの小定理"
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for ModInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let inv = Self::inv(other.0);
        self * inv
    }
}

impl DivAssign for ModInt {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

use std::convert::TryInto;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Idx2D {
    pub x: usize,
    pub y: usize,
    h: usize,
    w: usize,
}

impl Idx2D {
    pub fn new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Self {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        assert!(x >= 0 && y >= 0);
        assert!(x < w as isize && y < h as isize);

        Idx2D {
            x: x as usize,
            y: x as usize,
            h,
            w,
        }
    }

    pub fn try_new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Option<Self> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        if x >= 0 && y >= 0 && x < w as isize && y < h as isize {
            Some(Self {
                x: x as usize,
                y: y as usize,
                w,
                h,
            })
        } else {
            None
        }
    }

    pub fn try_add<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Self> {
        let dx = rhs.0.try_into().ok().unwrap();
        let dy = rhs.1.try_into().ok().unwrap();
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;

        Self::try_new(x, y, self.w, self.h)
    }

    pub fn try_sub<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Self> {
        let dx = rhs.0.try_into().ok().unwrap();
        let dy = rhs.1.try_into().ok().unwrap();
        let x = self.x as isize - dx;
        let y = self.y as isize - dy;

        Self::try_new(x, y, self.w, self.h)
    }

    pub fn neighber4(&self) -> impl Iterator<Item = Idx2D> {
        const VECT: &[(isize, isize)] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];
        let iter = *self;
        VECT.iter().filter_map(move |u| iter.try_add(u.clone()))
    }

    pub fn neighber8(&self) -> impl Iterator<Item = Idx2D> {
        const VECT: &[(isize, isize)] = &[
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        let iter = *self;
        VECT.iter().filter_map(move |u| iter.try_add(u.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Board<T>(pub Vec<Vec<T>>);

impl<T> Board<T> {
    pub fn new(b: Vec<Vec<T>>) -> Self {
        Self(b)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn idx(&self, x: usize, y: usize) -> Idx2D {
        Idx2D::new(x, y, self.width(), self.height())
    }

    pub fn get<Ix: TryInto<isize>>(&self, x: Ix, y: Ix) -> Option<&T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.width() as isize;
        let h = self.height() as isize;
        if x >= 0 && y >= 0 && x < w && y < h {
            Some(&self.0[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_mut<Ix: TryInto<isize>>(&mut self, x: Ix, y: Ix) -> Option<&mut T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.width() as isize;
        let h = self.height() as isize;
        if x >= 0 && y >= 0 && x < w && y < h {
            Some(&mut self.0[y as usize][x as usize])
        } else {
            None
        }
    }
}

impl<T: PartialEq> Board<T> {
    pub fn find(&self, v: T) -> Option<Idx2D> {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if *self.get(j, i).unwrap() == v {
                    return Some(self.idx(i, j));
                }
            }
        }
        None
    }
}

impl<T> Index<Idx2D> for Board<T> {
    type Output = T;

    fn index(&self, ix: Idx2D) -> &Self::Output {
        &self.0[ix.y][ix.x]
    }
}

impl<T> IndexMut<Idx2D> for Board<T> {
    fn index_mut(&mut self, ix: Idx2D) -> &mut Self::Output {
        &mut self.0[ix.y][ix.x]
    }
}

//
pub fn binary_search<T, F>(l: T, r: T, pred: F) -> T
where
    T: Add<Output = T> + Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;
    let two = T::from(2u8);

    loop {
        let m = (l + r) / two;
        if l == m || r == m {
            break l;
        }
        if pred(m) {
            l = m;
        } else {
            r = m;
        }
    }
}

/// vの要素について最初に現れたval以上の要素のインデックスを返す
pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] < *val) + 1) as usize
}

/// vの要素について最初に現れたvalより大きい要素のインデックスを返す
pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] <= *val) + 1) as usize
}

fn main() {}
