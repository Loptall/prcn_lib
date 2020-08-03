#![allow(unused_imports)]

// language: Rust(1.42.0)
// check available crates on AtCoder at "https://atcoder.jp/contests/language-test-202001"
// My Library Repositry is at "https://github.com/Loptall/sfcpl"

/*
    青perfほしいよ〜！
*/

// from std...
use std::cmp::{
    max, min, Ordering,
    Ordering::{Equal, Greater, Less},
    Reverse,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt;
use std::mem::swap;
use std::num::{NonZeroU32, ParseIntError};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, IndexMut, Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign,
};
use std::process::exit;
use std::{f32, f64, i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

// Dep' crates are...
use itertools::*;
use itertools_num::*;
use lazy_static::lazy_static;
use maplit::{btreemap, btreeset, hashmap, hashset};
use num_bigint::{BigInt, BigUint};
use num_complex::Complex;
use num_integer::{binomial, gcd, lcm, multinomial, Integer};
use num_rational::Rational;
use num_traits::{
    clamp, one, pow, zero, Num, NumAssignOps, NumOps, One, Pow, Signed, Unsigned, Zero,
};
use permutohedron::Heap;
use proconio::{
    derive_readable, fastout, input, is_stdin_empty,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use rand::random;

pub trait Visualize {
    fn visualize(&self, split: &str);
    fn continuous(&self) {
        self.visualize("");
    }
    fn spaces(&self) {
        self.visualize(" ");
    }
    fn lines(&self) {
        self.visualize("\n");
    }
}
macro_rules! impl_vis_for_sized {
    ($($t:ty),+) => {
        $(
            impl Visualize for $t {
                fn visualize(&self, _split: &str) {
                    print!("{}", self);
                }
            }
        )+
    };
}
impl_vis_for_sized! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f64, f32,
    String, &str, char
}
impl<T: fmt::Display> Visualize for [T] {
    fn visualize(&self, split: &str) {
        print!("{}", self.iter().join(split));
    }
}
#[macro_export]
macro_rules! vis {
    // end
    () => {
        println!();
    };

    // last element + trailing pattern
    ($last:expr ;) => {
        $last.lines();
        vis!()
    };
    ($last:expr =>) => {
        $last.continuous();
        vis!();
    };
    ($last:expr $(,)?) => {
        $last.spaces();
        vis!();
    };

    // get first element and pass rest
    ($first:expr; $($rest:tt)*) => {
        $first.lines();
        println!();
        vis!($($rest)*);
    };
    ($first:expr => $($rest:tt)*) => {
        $first.continuous();
        vis!($($rest)*);
    };
    ($first:expr, $($rest:tt)*) => {
        $first.spaces();
        print!(" ");
        vis!($($rest)*);
    };
}
pub const MOD10E9_7: usize = 1000000007; // 10 ^ 9 + 7
pub const MOD99_: usize = 998244353;
pub const MAX: usize = std::usize::MAX; // = 2 ^ 64 - 1 = 18446744073709551615 ≈ 1.8 * 10 ^ 19
pub const INF: usize = 2000000000000000000; // MAX / 9 < 2 * 10e18 < MAX / 10
pub const FNI: i64 = -2000000000000000000; // == -(INF as i64)
pub const PI: f64 = f64::consts::PI; // 3.141592653589793 -- 10 ^ -15
pub const ASCII_A_LARGE: u8 = 65;
pub const ASCII_A_SMALL: u8 = 97;
pub const ASCII_0: u8 = 48;
pub const ADJ4: &[(isize, isize); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const ADJ8: &[(isize, isize); 8] = &[
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<usize>>(),
            size: vec![1; n],
        }
    }
    pub fn len(&self) -> usize {
        self.parent.len()
    }
    /// i が属する集合の親のインデックス
    pub fn find(&mut self, mut i: usize) -> usize {
        while self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
            i = self.parent[self.parent[i]];
        }
        i
    }
    /// a と b を繋ぐ
    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.size[a] += self.size[b];
        self.parent[b] = a;
        true
    }
    /// i が属する集合の要素数
    pub fn count(&mut self, i: usize) -> usize {
        let p = self.find(i);
        self.size[p]
    }
    /// a と b が同一集合に属するか
    pub fn joint(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    /// i が属する集合の要素を列挙する
    pub fn get_group(&mut self, i: usize) -> HashSet<usize> {
        let p = self.find(i);
        (0..self.len()).filter(|x| self.find(*x) == p).collect()
    }
}

// code...
// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        bridges: [(Usize1, Usize1); m]
    }

    let mut uf = UnionFind::new(n);

    let mut ans: Vec<usize> = Vec::new();

    for (from, to) in bridges.into_iter().rev() {
        // size で書き換えれる
        if uf.joint(from, to) {
            ans.push(0);
        } else {
            ans.push(uf.count(from) * uf.count(to));
            uf.unite(from, to);
        }
    }

    ans.reverse();
    let mut i = 0;
    for j in ans {
        i += j;
        vis!(i);
    }
}
