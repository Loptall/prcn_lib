use std::io::stdin;
use std::str::FromStr;

use num::Integer;

pub fn input<T: FromStr>() -> T {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

pub fn input_vec<T: FromStr>() -> Vec<T> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect::<Vec<_>>()
}

pub fn input_chars() -> Vec<char> {
    let s: String = input();
    s.chars().collect()
}

pub fn input_lines<T: FromStr>(l: usize) -> Vec<T> {
    let mut res = Vec::new();
    for _i in 0..l {
        res.push(input());
    }
    res
}

pub fn input_lines_two<T: FromStr, U: FromStr>(l: usize) -> Vec<(T, U)> {
    let mut res = Vec::new();
    for _i in 0..l {
        res.push(input_two());
    }
    res
}

pub fn input_lines_three<T: FromStr, U: FromStr, V: FromStr>(l: usize) -> Vec<(T, U, V)> {
    let mut res = Vec::new();
    for _i in 0..l {
        res.push(input_three());
    }
    res
}

pub fn input_two<T: FromStr, U: FromStr>() -> (T, U) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let s: Vec<&str> = buf.split_whitespace().collect();
    (
        s[0].parse::<T>().ok().unwrap(),
        s[1].parse::<U>().ok().unwrap(),
    )
}

pub fn input_three<T: FromStr, U: FromStr, V: FromStr>() -> (T, U, V) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let s: Vec<&str> = buf.split_whitespace().collect();
    (
        s[0].parse::<T>().ok().unwrap(),
        s[1].parse::<U>().ok().unwrap(),
        s[2].parse::<V>().ok().unwrap(),
    )
}

pub fn input_four<T: FromStr, U: FromStr, V: FromStr, W: FromStr>() -> (T, U, V, W) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let s: Vec<&str> = buf.split_whitespace().collect();
    (
        s[0].parse().ok().unwrap(),
        s[1].parse().ok().unwrap(),
        s[2].parse().ok().unwrap(),
        s[3].parse().ok().unwrap(),
    )
}

pub fn input_matrix<T: FromStr + Clone>(h: usize) -> Vec<Vec<T>> {
    let mut res = vec![Vec::new(); h];
    for i in 0..h {
        res[i] = input_vec();
    }
    res
}


#[derive(Debug, Clone)]
pub struct SegmentTree<T: Monoid> {
    leaves: usize,
    size: usize,
    value: Vec<T>,
}

fn childrens_idx(n: usize) -> (usize, usize) {
    (n * 2 + 1, n * 2 + 2)
}

fn parent_idx(n: usize) -> usize {
    (n - 1) / 2
}

impl<T: Monoid + Clone + Copy> SegmentTree<T> {
    pub fn new(v: &[T]) -> Self {
        let n = v.len();
        let leaves = n.next_power_of_two();
        let size = 2 * leaves - 1;
        let mut value = vec![T::identity(); size];

        for i in (0..size).rev() {
            if i >= leaves - 1 {
                if i + 1 - leaves < n {
                    value[i] = T::from(v[i + 1 - leaves]);
                } else {
                    continue;
                }
            } else {
                let (left, right) = childrens_idx(i);
                value[i] = T::op(&value[left], &value[right]);
            }
        }

        Self {
            leaves,
            size,
            value,
        }
    }

    fn childrens(&self, n: usize) -> (T, T) {
        let (left, right) = childrens_idx(n);
        (self.value[left], self.value[right])
    }

    pub fn get_raw(&self, i: usize) -> Option<T> {
        if self.leaves <= i {
            panic!("too big index")
        }
        Some(self.value[self.size - i - 1])
    }

    pub fn update(&mut self, i: usize, v: T) {
        let mut cur = self.leaves - 1 + i;
        self.value[cur] = v;
        loop {
            cur = parent_idx(cur);
            let (left, right) = self.childrens(cur);
            self.value[cur] = T::op(&left, &right);
            if cur == 0 {
                break;
            }
        }
    }

    pub fn range(&self, from: usize, to: usize) -> T {
        self.range_sub(from, to, 0, self.leaves, 0)
    }

    fn range_sub(&self, from: usize, to: usize, l_bound: usize, r_bound: usize, k: usize) -> T {
        if from <= l_bound && to >= r_bound {
            self.value[k]
        } else if from >= r_bound || to <= l_bound {
            T::identity()
        } else {
            let sep = (l_bound + r_bound) / 2;
            T::op(
                &self.range_sub(from, to, l_bound, sep, 2 * k + 1),
                &self.range_sub(from, to, sep, r_bound, 2 * k + 2),
            )
        }
    }
}

use std::cmp::max;

fn main() {
    let n: usize = input();
    let a: Vec<usize> = input_vec();

    let s = SegmentTree::<Gcd<usize>>::new(&a.iter().map(|x| Gcd(*x)).collect::<Vec<_>>());

    let mut ans = 0usize;
    for rem in 0..n {
        ans = max(ans, Gcd::op(&s.range(0, rem), &s.range(rem + 1, s.leaves)).0)
    }

    println!("{}", ans);
}


/// 単位元が定義される `T -> T -> T`型の演算
pub trait Monoid: Sized {
    fn identity() -> Self;

    fn op(x: &Self, y: &Self) -> Self;

    fn fold(v: &[Self]) -> Self {
        v.iter().fold(Self::identity(), |a, b| Self::op(&a, b))
    }
}

/// 一行目にモノイド名、
/// 二行目に単位元
/// 三行目に`x`と`y`を引数に取って、同じ型の演算結果を返すクロージャを渡す
#[macro_export]
macro_rules! monoid_def {
    {
        $M:ident<$t:ty>,
        $id:expr,
        $me:expr
    } => {
        #[derive(Debug, Clone, Copy)]
        pub struct $M($t);

        impl Monoid for $M {
            fn identity() -> Self {
                $M($id)
            }

            fn op(x: &Self, y: &Self) -> Self {
                let f = $me;
                $M(f(x.0, y.0))
            }
        }
    };
}


#[derive(Debug, Clone, Copy)]
pub struct Gcd<T: Integer>(T);

impl<T: Integer> Monoid for Gcd<T> {
    fn identity() -> Self {
        Gcd(T::zero())
    }

    fn op(x: &Self, y: &Self) -> Self {
        Gcd(x.0.gcd(&y.0))
    }
}

impl<T: Integer> From<T> for Gcd<T> {
    fn from(x: T) -> Self {
        Gcd(x)
    }
}
