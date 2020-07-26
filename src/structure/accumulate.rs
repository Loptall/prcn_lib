use cargo_snippet::snippet;

use num::traits::identities::{zero, Zero};
use std::ops::{Add, Index, Sub};

/// 累積和型
///
/// 初期化 O(n)
/// 区間 O(1)
/// 更新は作り直す以外に出来ない
///
/// 逆操作はimos法,
#[snippet("accumulate")]
#[derive(PartialEq, Clone, Debug)]
pub struct Accumulate<T>(Vec<T>);

#[snippet("accumulate")]
impl<T: Zero + Copy + Add<Output = T> + Sub<Output = T>> Accumulate<T> {
    /// `[0, i)`の総和
    pub fn sum(&self, i: usize) -> T {
        self[i]
    }

    /// 累積和を利用して`[i, j)`間の総和を`O(1)`で求める
    pub fn pertical_sum(&self, i: usize, j: usize) -> T {
        self[j] - self[i]
    }
}

#[snippet("accumulate")]
impl<T: Zero + Add<Output = T> + Copy> Index<usize> for Accumulate<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[snippet("accumulate")]
/// 累積和をとります
///
/// `O(n)`
pub fn accumulate<T: Zero + Copy + Add<Output = T> + Sub<Output = T>>(v: &[T]) -> Accumulate<T> {
    let mut res = Vec::with_capacity(v.len() + 1);
    let mut i: T = zero();
    res.push(i);
    for e in v {
        i = i + *e;
        res.push(i);
    }
    Accumulate(res)
}

#[test]
fn acum_test() {
    let v = vec![1, 2, 3, 4, 5];
    let acm = accumulate(&v);
    assert_eq!(acm, Accumulate(vec![0, 1, 3, 6, 10, 15]));
}

#[test]
fn sum_test() {
    let v = vec![1, 2, 3, 4, 5];
    let acm = accumulate(&v);
    assert_eq!(acm.pertical_sum(0, 3), 6);
    assert_eq!(acm.pertical_sum(4, 5), 5);
}
