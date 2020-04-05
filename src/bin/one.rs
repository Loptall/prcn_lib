// from std
pub use std::cmp::{max, min};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

// from num-integer
// binomial(n, r)     => nCr
// n.gcd(&m)          => gcd(n, m)
// n.lcm(&m)          => lcm(n, m)
// a.extended_gcd(&b) => Extended(gcd(a, b), x, y, ()) // a * x + b * y = gcd(a, b) の唯一解
pub use num::integer::binomial;
pub use num::Integer;

// from num-bigint
pub use num::{BigInt, BigUint};

// from random
pub use rand::random;

// from proconio
pub use proconio::{derive_readable, fastout, input, is_stdin_empty, marker::*};

// from itertools
pub use itertools::*;

// from maplit
pub use maplit::{btreemap, btreeset, hashmap, hashset};

use num::traits::identities::Zero;
use num::zero;
use std::ops::{AddAssign, Index, Sub};

/// 累積和型
#[derive(PartialEq, Clone, Debug)]
pub struct Accumulate<T>(Vec<T>);

impl<T: Zero + Copy + AddAssign + Sub<Output = T>> Accumulate<T> {
    /// 累積和をとります
    pub fn accumulate(v: &[T]) -> Self {
        let mut res = Vec::with_capacity(v.len() + 1);
        let mut i: T = zero();
        res.push(i);
        for e in v {
            i += *e;
            res.push(i);
        }
        Accumulate(res)
    }

    /// 累積和を利用して`[i, j)`間の総和を`O(1)`で求める
    pub fn get_pertical_sum(&self, i: usize, j: usize) -> T {
        self[j] - self[i]
    }
}

impl<T: Zero + AddAssign + Copy> Index<usize> for Accumulate<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[test]
fn acum_test() {
    let v = vec![1, 2, 3, 4, 5];
    let acm = Accumulate::accumulate(&v);
    assert_eq!(acm, Accumulate(vec![0, 1, 3, 6, 10, 15]));
}

#[test]
fn sum_test() {
    let v = vec![1, 2, 3, 4, 5];
    let acm = Accumulate::accumulate(&v);
    assert_eq!(acm.get_pertical_sum(0, 3), 6);
    assert_eq!(acm.get_pertical_sum(4, 5), 5);
}

use std::ops::{Add, Div};

/// `l`が`pred`に対し｀true`, `r`が`pred`に対し`false`を返す時
/// `l`と`r`の中間の値のうちで最後に`pred`を満たすものを返す
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

#[test]
fn binary_search_test() {
    let v = vec![1, 3, 3, 3, 4, 6, 7, 7, 8, 10];
    // println!("{}", -1 / 2);
    assert_eq!(4, binary_search(v.len() as _, -1, |i| v[i as usize] > 3));
    assert_eq!(1, binary_search(v.len() as _, -1, |i| v[i as usize] >= 3));
}

/// vの要素について最初に現れたval以上の要素のインデックスを返す
pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] < *val) + 1) as usize
}

#[test]
fn lower_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(lower_bound(v, &0), 0);
    assert_eq!(lower_bound(v, &1), 0);
    assert_eq!(lower_bound(v, &2), 1);
    assert_eq!(lower_bound(v, &3), 1);
    assert_eq!(lower_bound(v, &4), 3);
    assert_eq!(lower_bound(v, &5), 4);
    assert_eq!(lower_bound(v, &999), 5);
}

/// vの要素について最初に現れたvalより大きい要素のインデックスを返す
pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] <= *val) + 1) as usize
}

#[test]
fn upper_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(upper_bound(v, &0), 0);
    assert_eq!(upper_bound(v, &1), 1);
    assert_eq!(upper_bound(v, &2), 1);
    assert_eq!(upper_bound(v, &3), 3);
    assert_eq!(upper_bound(v, &4), 4);
    assert_eq!(upper_bound(v, &5), 5);
    assert_eq!(upper_bound(v, &999), 5);
}
pub const ATCODER: &str = "atcoder";

pub trait Graph<'a> {
    type NodeId: Copy;
    type Iter: Iterator<Item = Self::NodeId>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self, a: Self::NodeId) -> usize;
    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter;
}

/// 重みなしグラフ
pub type UnweightedGraph = Vec<Vec<usize>>;

impl<'a> Graph<'a> for UnweightedGraph {
    type NodeId = usize;
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self[a].iter().cloned()
    }
}

/// 重みなし有向グラフ
pub fn make_directed_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
    }
    g
}

/// 重みなし無向グラフ
pub fn make_undirected_graph(n: usize, edges: &[(usize, usize)]) -> UnweightedGraph {
    let mut g = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        g[u].push(v);
        g[v].push(u);
    }
    g
}

/// 重みありグラフ
pub type WeightedGraph<W> = Vec<Vec<(usize, W)>>;

impl<'a, W> Graph<'a> for WeightedGraph<W>
where
    W: std::marker::Copy + Clone + 'a,
{
    type NodeId = (usize, W);
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index(&self, a: Self::NodeId) -> usize {
        a.0
    }

    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self[a.0].iter().cloned()
    }
}

/// 重み付き有向グラフ
pub fn make_weighted_directed_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
    }
    g
}

/// 重み付き無向グラフ
pub fn make_weighted_undirected_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }
    g
}

pub struct Dfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    s: Vec<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

impl<'a, G: Graph<'a>> Iterator for Dfs<'a, G> {
    type Item = (G::NodeId, G::NodeId);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.s.pop() {
            for v in self.g.neighbors(u) {
                if !self.visited[self.g.index(v)] {
                    self.visited[self.g.index(v)] = true;
                    self.s.push((v, Some(u)));
                }
            }

            if let Some(prev) = prev {
                Some((prev, u))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn dfs<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Dfs<'a, G> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut s = Vec::new();
    visited[start] = true;
    s.push((start, None));

    Dfs { visited, s, g }
}

pub struct Bfs<'a, G: Graph<'a>> {
    visited: Vec<bool>,
    q: VecDeque<(G::NodeId, Option<G::NodeId>)>,
    g: &'a G,
}

impl<'a, G: Graph<'a>> Iterator for Bfs<'a, G> {
    type Item = (G::NodeId, G::NodeId);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.q.pop_front() {
            for v in self.g.neighbors(u) {
                if !self.visited[self.g.index(v)] {
                    self.visited[self.g.index(v)] = true;
                    self.q.push_back((v, Some(u)));
                }
            }

            if let Some(prev) = prev {
                Some((prev, u))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn bfs<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Bfs<'a, G> {
    let n = g.len();
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[start] = true;
    q.push_back((start, None));

    Bfs { visited, q, g }
}

/// Returns a vector which stores distances from `start`.
/// For unreachable node, `usize::MAX` is stored.
pub fn make_dist_table<'a, G: Graph<'a, NodeId = usize>>(g: &'a G, start: G::NodeId) -> Vec<usize> {
    let mut dist = vec![std::usize::MAX; g.len()];
    dist[start] = 0;
    for (u, v) in bfs(g, start) {
        dist[v] = dist[u] + 1;
    }
    dist
}
// Varified

use std::convert::TryInto;
use std::ops::IndexMut;

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
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

impl<T: PartialEq> Grid<T> {
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

impl<T> Index<Idx2D> for Grid<T> {
    type Output = T;

    fn index(&self, ix: Idx2D) -> &Self::Output {
        &self.0[ix.y][ix.x]
    }
}

impl<T> IndexMut<Idx2D> for Grid<T> {
    fn index_mut(&mut self, ix: Idx2D) -> &mut Self::Output {
        &mut self.0[ix.y][ix.x]
    }
}

// Varified

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
            y: y as usize,
            w,
            h,
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

use num::one;
use std::convert::{From, Into};
use std::ops::{DivAssign, Mul, MulAssign};
// use num::BigInt;

/// `n`を十進法で表したときの桁数
pub fn scale_dight<T: Integer + DivAssign + Mul + From<u8>>(n: T) -> usize {
    if n == zero() {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= one() {
        n /= one::<T>() * 10u8.into();
        count += 1;
    }
    count
}

#[test]
fn test_scale() {
    let a = 99i64;
    assert!(2 == scale_dight(a));
    let b = 0usize;
    assert!(1 == scale_dight(b));
    let c: num::BigInt = num::BigInt::from(21_746_284_928_973_i128);
    assert!(14 == scale_dight(c));
}

/// `n`を`base`進法で表したときの桁数
pub fn scale_n_base<T: Integer + DivAssign + Mul + From<u8>>(n: T, base: u8) -> usize {
    if n == zero() {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= one() {
        n /= one::<T>() * base.into();
        count += 1;
    }
    count
}

#[test]
fn test_n_base_scale() {
    let a = 99i64;
    assert_eq!(7, scale_n_base(a, 2));
    let b = 0usize;
    assert_eq!(1, scale_n_base(b, 100));
    let c: num::BigInt =
        num::BigInt::from(21_746_284_928_973_i128) * num::BigInt::from(11_111_111_111_111_111_i128);
    assert_eq!(35, scale_n_base(c, 7));
}

/// 整数を桁ごとに`Vec<u64>`に分解
pub fn dight_vec<
    T: std::ops::MulAssign + std::convert::From<u8> + std::ops::DivAssign + Integer + Copy,
>(
    n: T,
) -> Vec<T> {
    let mut idx = scale_dight(n) - 1;
    let mut ret = Vec::new();
    loop {
        ret.push(n / pow_bin(10.into(), idx as u32) % 10.into());
        if idx == 0 {
            break;
        }
        idx -= 1;
    }
    ret
}

#[test]
fn vec_test() {
    let a = dight_vec(12345usize);
    assert_eq!(a, vec![1, 2, 3, 4, 5]);
}

/// 整数の十進法での各桁の和
pub fn dight_sum<
    T: std::ops::MulAssign + std::convert::From<u8> + std::ops::DivAssign + Integer + Copy + AddAssign,
>(
    n: T,
) -> T {
    let mut res = zero();
    for i in dight_vec(n) {
        res += i;
    }
    res
}

/// 二分累乗法,
/// - `O(log n)`で累乗を求める
pub fn pow_bin<T: Integer + MulAssign + Copy>(n: T, r: u32) -> T {
    let mut res: T = one();
    let mut a = n;
    let mut n = r;
    while n > 0 {
        if n & 1 != 0 {
            res *= a
        }
        a *= a;
        n >>= 1;
    }
    res
}

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
                res = res * a.0 % Self::MOD;
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

use std::ops::SubAssign;

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

#[test]
fn add() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a + b).0,
            ((a.0 as u128 + b.0 as u128) % (ModInt::MOD as u128)) as u64
        )
    }
}

#[test]
fn sub() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a - b).0,
            if ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) >= 0 {
                ((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) as u64
            } else {
                (((a.0 as i128 - b.0 as i128) % (ModInt::MOD as i128)) + ModInt::MOD as i128) as u64
            }
        )
    }
}

#[test]
fn mul() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!(
            (a * b).0,
            ((a.0 as u128 * b.0 as u128) % (ModInt::MOD as u128)) as u64
        )
    }
}

#[test]
fn div() {
    for _i in 0..10000 {
        let a = ModInt::new(random::<u64>());
        let b = ModInt::new(random::<u64>());
        assert_eq!((a / b).0 * b.0 % ModInt::MOD, a.0)
    }
}

#[test]
fn fac_test() {
    assert_eq!(6, ModInt::new(3).factorial().0)
}

/// `i`が素数 <=> `self.0[i]`
#[derive(Debug)]
pub struct Prime(Vec<bool>);

impl Prime {
    /// エラストテネスの篩を用いて`n`より小さい素数を調べます
    /// 大体`O(n log log n)`なはず
    ///
    /// AtCoder Language Test 202001 コードテスト上で
    /// 10^8が限界 <- 十分
    pub fn init(n: usize) -> Self {
        let mut sieve = vec![true; n];
        sieve[0] = false;
        sieve[1] = false;

        let mut i = 2;
        while i * i <= n {
            if sieve[i] {
                (2..)
                    .map(|x| x * i)
                    .take_while(|x| *x < n)
                    .for_each(|x| sieve[x] = false);
            }
            i += 1;
        }

        Self(sieve)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// 前計算によるテーブルを用いて素因数分解をします
    ///
    /// 返り値は`Vec<(素因数, 指数)>`の形式
    ///
    /// 十分に大きいテーブルを渡してください
    ///
    /// # Panic
    /// `self`の最大値が`n`より大きいときパニックする
    ///
    /// ```
    /// let table = Prime::init(10);
    /// assert_eq!(table.factorization(9), vec![(3, 2)]); // ok!
    /// // assert_eq!(table.factorization(10), vec![(2, 1), (5, 1)]); // panic! Because table.len() <= n
    /// ```
    pub fn factorization(&self, n: usize) -> Vec<(usize, usize)> {
        assert!(self.len() > n);

        let mut res = Vec::new();
        let ps = self.primes();
        let mut n = n;
        for p in ps {
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                res.push((p, count));
            }
        }
        if n > 1 {
            res.push((n, 1));
        }
        res
    }

    pub fn primes(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|t| *t.1)
            .map(|x| x.0)
            .collect()
    }

    /// `n`が素数かどうかをテーブルを用いて判定します
    ///
    /// `O(1)`
    ///
    /// # Panic
    /// `n >= self.0.len()`のとき、panicします。
    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n < self.len());

        self.0[n]
    }
}

#[test]
fn sieve_test() {
    let p = Prime::init(10);
    assert_eq!(
        &[false, false, true, true, false, true, false, true, false, false],
        &p.0[..]
    );
}

#[test]
fn fact_test() {
    let table = Prime::init(10);
    assert_eq!(table.factorization(9), vec![(3, 2)]); // ok!
                                                      // assert_eq!(table.factorization(10), vec![(2, 1), (5, 1)]); // panic! Because table.len() <= n
}

/// 素因数分解をします。
///
/// 呼び出すごとに新しく篩を作るので
/// 複数回を呼び出すときは計算量が増大します
/// 初期化したPrime構造体に対してメゾットを呼んでください。
/// (その代わりにパニックしません)
pub fn factorization(n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let ps = Prime::init(n).primes();
    let mut n = n;
    for p in ps {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        if count > 0 {
            res.push((p, count));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };

    for n in (3..).step_by(2).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }

    x
}

/// 試し割りによる素数判定です
///
/// `O(√n)`
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}

pub fn shuffle_vec<T>(v: &mut Vec<T>) {
    let len = v.len();
    for i in 0..100 {
        let a: usize = random::<usize>() % len;
        let b: usize = random::<usize>() % len;
        if a == b {
            continue;
        }
        if i % 2 == 0 {
            v[a.min(b)..=b.max(a)].rotate_left((a.max(b) - a.min(b)) / 2);
        } else {
            v[a.min(b)..=b.max(a)].rotate_right((a.max(b) - a.min(b)) / 2);
        }
    }
}

#[test]
fn test_shuffle() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    shuffle_vec(&mut v);
    println!("{:?}", v);
}
// Varified

/// 配列に含まれる要素(T: Ord + Copy)別にその個数を数えて
/// `BTreeMap<T, usize>`の形にして返す
pub fn count_element_to_map<T: Ord + Copy>(v: &[T]) -> BTreeMap<T, usize> {
    let mut map = BTreeMap::new();
    for e in v {
        let h = map.entry(*e).or_insert(0);
        *h += 1;
    }

    map
}

#[test]
fn count_test() {
    let v = vec![1, 2, 2, 3, 3, 3, 4, 5, 7];
    let map = count_element_to_map(&v);
    assert_eq!(
        map,
        maplit::btreemap![1 => 1, 2 => 2, 3 => 3, 4 => 1, 5 => 1, 7 => 1]
    );
}

/// プログラムを終了
/// 単純に`std::process::exit(0)`の省略型
pub fn exit() {
    use std::process;
    process::exit(0)
}

fn main() {}
