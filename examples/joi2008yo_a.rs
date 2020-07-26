#![allow(unused_imports)]

// language: Rust(1.42.0)
// check available crates on AtCoder at "https://atcoder.jp/contests/language-test-202001"
// My Library Repositry is at "https://github.com/Loptall/prcn_lib"

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
pub const INF: usize = 20000000000000; // MAX / 9 < 2 * 10e18 < MAX / 10
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

// code...
// #[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    }

    let mut g = make_graph_for_dijkstra(n, &Vec::new());

    for _ in 0..k {
        input! {
            q: u8
        }

        if q == 0 {
            input! {
                from: Usize1, to: Usize1
            }

            let ans = dijkstra(&g, from)[to];
            vis!(if ans >= INF as u64 {
                "-1".to_string()
            } else {
                ans.to_string()
            });
        } else {
            input! {
                from: Usize1, to: Usize1, w: u64
            }
            g.add_edge((from, to, w));
        }
    }
}

pub trait Graph<'a> {
    type NodeId: Copy;
    type Iter: Iterator<Item = Self::NodeId>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self, i: Self::NodeId) -> usize;
    fn neighbors(&'a self, i: Self::NodeId) -> Self::Iter;
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
/// 重みありグラフ
pub type WeightedNodeGraph<W> = Vec<Vec<(usize, W)>>;
impl<'a, W> Graph<'a> for WeightedNodeGraph<W>
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
/// 重み付き有向グラフ
pub fn make_weighted_directed_graph<W: Clone>(
    n: usize,
    edges: &[(usize, usize, W)],
) -> WeightedNodeGraph<W> {
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
) -> WeightedNodeGraph<W> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in edges.iter() {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }
    g
}

/// 単純グラフじゃないと死ぬ
pub struct Dijkstra {
    g: Vec<Vec<usize>>,
    e: HashMap<(usize, usize), u64>,
}
impl<'a> Graph<'a> for Dijkstra {
    type NodeId = usize;
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Self::NodeId>>;
    fn len(&self) -> usize {
        self.g.len()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn index(&self, a: Self::NodeId) -> usize {
        a
    }
    fn neighbors(&'a self, a: Self::NodeId) -> Self::Iter {
        self.g[a].iter().cloned()
    }
}
pub fn make_graph_for_dijkstra(n: usize, edges: &[(usize, usize, u64)]) -> Dijkstra {
    let mut g = vec![Vec::new(); n];
    let mut e = HashMap::new();
    for (from, to, weight) in edges.iter() {
        g[*from].push(*to);
        g[*to].push(*from);
        e.insert((*from, *to), *weight);
        e.insert((*to, *from), *weight);
    }
    Dijkstra { g, e }
}
impl<'a> Dijkstra {
    pub fn weight(&self, from: usize, to: usize) -> u64 {
        self.e[&(from, to)]
    }
    pub fn add_edge(&mut self, edge: (usize, usize, u64)) {
        if self.g[edge.0].contains(&edge.1) {
            *self.e.get_mut(&(edge.0, edge.1)).unwrap() = min(edge.2, self.weight(edge.0, edge.1));
            *self.e.get_mut(&(edge.1, edge.0)).unwrap() = min(edge.2, self.weight(edge.1, edge.0));
        } else {
            self.g[edge.0].push(edge.1);
            self.g[edge.1].push(edge.0);
            self.e.insert((edge.0, edge.1), edge.2);
            self.e.insert((edge.1, edge.0), edge.2);
        }
    }
}
/// 任意の頂点から全ての頂点までの最短経路を求める
/// `O((V + E) log V)`
pub fn dijkstra<'a>(g: &'a Dijkstra, start: usize) -> Vec<u64> {
    let mut d = Vec::with_capacity(g.len());
    let mut q: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::with_capacity(g.len());
    for i in 0..g.len() {
        let j = if i == start { 0 } else { INF as u64 };
        d.push(j);
        q.push(Reverse((j, i)));
    }
    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors(u.1) {
            let w = g.weight(u.1, v);
            if d[v] > d[u.1] + w {
                d[v] = d[u.1] + w;
                q.push(Reverse((w, v)));
            }
        }
    }
    d
}
/// 二点間の最短距離とその経路を求める
pub fn dijkstra_with_path<'a>(g: &'a Dijkstra, start: usize, goal: usize) -> (Vec<usize>, u64) {
    let mut d = Vec::with_capacity(g.len());
    let mut q: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::with_capacity(g.len());
    for i in 0..g.len() {
        let j = if i == start { 0 } else { INF as u64 };
        d.push(j);
        q.push(Reverse((j, i)));
    }
    let mut p: Vec<Option<usize>> = vec![None; g.len()];
    while !q.is_empty() {
        let u = q.pop().unwrap().0;
        for v in g.neighbors(u.1) {
            let w = g.weight(u.1, v);
            if d[v] > d[u.1] + w {
                d[v] = d[u.1] + w;
                p[v] = Some(u.1);
                q.push(Reverse((w, v)));
            }
        }
    }
    let mut path = vec![goal];
    loop {
        match p[*path.last().unwrap()] {
            Some(prev) if prev == start => {
                path.push(prev);
                break;
            }
            Some(prev) => {
                path.push(prev);
            }
            None => break,
        }
    }
    path.reverse();
    (path, d[goal])
}
