use cargo_snippet::snippet;
use num::Zero;
use std::ops::{Add, Sub};

// 逆操作が定まる(体?)であればgenericな定義が出来そう?

/// 更新、区間和の取得がともに`O(log n)`で
/// 可能なデータ構造
#[snippet("fenwick_tree")]
#[derive(Debug, Clone)]
pub struct FenwickTree<T: Add> {
    len: usize,
    segment: Vec<T>,
}

#[snippet("fenwick_tree")]
impl<T: Add + Sub<Output = T> + Zero + Clone + Ord + Into<usize>> FenwickTree<T> {
    /// サイズを指定して、zerosで初期化
    pub fn new(n: usize) -> Self {
        Self {
            len: n,
            segment: vec![T::zero(); n + 1],
        }
    }

    /// 配列から構築
    pub fn from(v: &[T]) -> Self {
        let n = v.len();
        let mut f = Self::new(n);
        for (i, v) in v.iter().enumerate() {
            f.add(i, v.clone());
        }
        f
    }

    /// 扱える要素の最大値
    pub fn len(&self) -> usize {
        self.len
    }

    /// # 更新
    /// `i`番目の要素 += `v`
    ///
    /// `O(log n)`
    pub fn add(&mut self, i: usize, v: T) {
        let mut i = i + 1;
        while i <= self.len() {
            self.segment[i] = self.segment[i].clone() + v.clone();
            i += (i as i64 & -(i as i64)) as usize;
        }
    }

    /// `[0..i)`の区間和
    ///
    /// `O(log n)`
    pub fn sum(&self, mut i: usize) -> T {
        let mut s = T::zero();
        while i > 0 {
            s = s.clone() + self.segment[i].clone();
            i -= (i as i64 & -(i as i64)) as usize;
        }
        s
    }

    /// `[a..b)`の区間和
    ///
    /// `O(log n)`
    pub fn partial_sum(&self, from: usize, to: usize) -> T {
        self.sum(to) - self.sum(from)
    }
}

#[test]
fn fenwick_test() {
    let v = vec![1usize, 2, 3, 4];
    let mut f = FenwickTree::from(&v);

    assert_eq!(f.sum(3), 6);
    assert_eq!(f.partial_sum(1, 3), 5);

    f.add(1, 100);

    assert_eq!(f.sum(2), 103);
}
