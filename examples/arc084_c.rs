use std::ops::{Add, Div};

use std::io::stdin;
use std::str::FromStr;

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

/// # Usage
///
/// `T`は基本的にusizeが入る想定
///
/// `pred`について`l`側が`true`、
/// `r`側が`false`を返すような単調性を持っているとき、
///
/// `pred`を満たす`T`の中で最も大きいものを求める。
///
/// 最初に`l`と`r`について`pred`がそれぞれ`true`、`false`を返すかどうかを調べるので、
/// 範囲は閉空間で指定して良い。
///
/// 返り値`res`は必ず、
///
/// Some(inner) (l <= inner <= r) | None
///
/// になる
pub fn binary_search<T, F>(pred: F, l: T, r: T) -> Option<T>
where
    T: Add<Output = T> + Div<Output = T> + PartialEq + Copy + From<u8>,
    F: Fn(T) -> bool,
{
    if !pred(l) {
        return None;
    }
    if pred(r) {
        return Some(r);
    }

    let mut l = l;
    let mut r = r;
    let two = T::from(2);

    loop {
        let m = (l + r) / two;
        if l == m {
            return Some(l);
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

    assert_eq!(3, binary_search(|i| v[i] < 4, 0, v.len() - 1).unwrap());
    assert_eq!(9, binary_search(|i| v[i] <= 10, 0, v.len() - 1).unwrap());
}

pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    let t = binary_search(|x| v[x] < *val, 0, v.len() - 1);
    match t {
        Some(e) => e + 1,
        None => 0,
    }
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

pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    let t = binary_search(|x| v[x] <= *val, 0, v.len() - 1);
    match t {
        Some(e) => e + 1,
        None => 0,
    }
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

fn main() {
    let n: usize = input();
    let mut a: Vec<usize> = input_vec();
    let b: Vec<usize> = input_vec();
    let mut c: Vec<usize> = input_vec();

    a.sort();
    c.sort();

    let mut ans = 0usize;
    for m in b {
        ans += lower_bound(&a, &m) * (n - upper_bound(&c, &m));
    }

    println!("{}", ans);
}
