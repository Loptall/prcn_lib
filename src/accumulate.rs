//! Varified!

use num::traits::identities::Zero;
use num::zero;
use std::ops::{AddAssign, Index, Sub};

/// 累積和型
#[derive(PartialEq, Clone, Debug)]
pub struct Accumulate<T>(Vec<T>);

impl<T: Zero + Copy + AddAssign + Sub<Output = T>> Accumulate<T> {
    /// 累積和をとります
    pub fn accumulate(v: &[T]) -> Self {
        let mut res = Vec::with_capacity(v.len());
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
