// 遅延伝播 Segment Tree
//
// 要素の集合 `T`, 作用素の集合 `E`
// が与えられたとき、
// モノイド(写像)、
// f: T x T -> T, (T の要素のマージ) / Sum だったり
// g: E x E -> E, (E の作用のマージ) / 区間add なら |e1, e2| -> e1 + e2 だし, 変更なら |e1, e2| -> e2
// h: T x E -> T, (E の T に対する作用: effect) / 区間add だったり..., 変更クエリだったり
// を、それぞれ二重線形性を満たした上で定義する。
// f と g は直接の関係はなし、独立に定義できる

use super::algebraic_traits::monoid::Monoid;

#[derive(Clone)]
pub struct DelayedSegmentTree<'a, T: Monoid, E: Monoid> {
    /// length of tree's leaves
    len: usize,
    /// length of whole vec
    size: usize,
    effect: &'a dyn Fn(T, E) -> T,
    segment: Vec<T>,
    lazy: Vec<E>,
}

fn childrens_idx(n: usize) -> (usize, usize) {
    (n * 2 + 1, n * 2 + 2)
}

impl<'a, T: Monoid + Clone + Copy, E: Monoid + Clone + Copy> DelayedSegmentTree<'a, T, E> {
    pub fn new<I: Into<T> + Copy>(v: &[I], effect: &'a dyn Fn(T, E) -> T) -> Self {
        let n = v.len();
        let len = n.next_power_of_two();
        let size = 2 * len - 1;
        let mut segment = vec![T::identity(); size];
        let lazy = vec![E::identity(); size];

        for i in (0..size).rev() {
            if i >= len - 1 {
                if i + 1 - len < n {
                    segment[i] = (v[i + 1 - len]).into();
                } else {
                    continue;
                }
            } else {
                let (left, right) = childrens_idx(i);
                segment[i] = T::op(&segment[left], &segment[right]);
            }
        }

        Self {
            len,
            size,
            effect,
            segment,
            lazy,
        }
    }

    /// number of LEAVES in tree
    pub fn len(&self) -> usize {
        self.len
    }

    /// length of vector
    pub fn size(&self) -> usize {
        self.size
    }

    // fn childrens(&self, n: usize) -> (T, T) {
    //     let (left, right) = childrens_idx(n);
    //     (self.segment[left], self.segment[right])
    // }

    /// `i`番目の葉の参照をとる
    pub fn get(&self, i: usize) -> Option<&T> {
        self.segment.get(self.size - self.len + i)
    }

    /// `i`番目の葉の可変参照をとる
    ///
    /// # Unsafe
    ///
    /// 葉の部分は変更されうるけれど、その親要素へ変更が伝達されない
    ///
    /// update()を使うこと
    pub unsafe fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.segment.get_mut(self.size - i - 1)
    }

    fn eval(&mut self, i: usize) {
        if self.lazy[i] == E::identity() {
            return;
        }

        if i < self.size() - 1 {
            let (left, right) = childrens_idx(i);
            self.lazy[left] = E::op(&self.lazy[left], &self.lazy[i]);
            self.lazy[right] = E::op(&self.lazy[right], &self.lazy[i]);
        }

        let f = self.effect;
        self.segment[i] = f(self.segment[i], self.lazy[i]);
        self.lazy[i] = E::identity();
    }

    pub fn update_range(&mut self, from: usize, to: usize, x: E) {
        self.update_range_inner(from, to, 0, self.size(), 0, x);
    }

    fn update_range_inner(
        &mut self,
        from: usize,
        to: usize,
        l_bound: usize,
        r_bound: usize,
        k: usize,
        x: E,
    ) {
        self.eval(k);
        if from >= r_bound || to <= l_bound {
            return;
        }
        if from <= l_bound && to >= r_bound {
            self.lazy[k] = E::op(&self.lazy[k], &x);
            self.eval(k);
            return;
        }

        let sep = (l_bound + r_bound) / 2;
        let (left, right) = childrens_idx(k);
        self.update_range_inner(from, to, l_bound, sep, left, x);
        self.update_range_inner(from, to, sep, r_bound, right, x);
        self.segment[k] = T::op(&self.segment[left], &self.segment[right]);
    }

    /// 区間、`[from..to)`を指定の`Monoid`でfoldした演算結果
    pub fn range(&mut self, from: usize, to: usize) -> T {
        self.range_inner(from, to, 0, self.len(), 0)
    }

    fn range_inner(
        &mut self,
        from: usize,
        to: usize,
        l_bound: usize,
        r_bound: usize,
        k: usize,
    ) -> T {
        self.eval(k);
        if from <= l_bound && to >= r_bound {
            self.segment[k]
        } else if from >= r_bound || to <= l_bound {
            T::identity()
        } else {
            let sep = (l_bound + r_bound) / 2;
            T::op(
                &self.range_inner(from, to, l_bound, sep, 2 * k + 1),
                &self.range_inner(from, to, sep, r_bound, 2 * k + 2),
            )
        }
    }
}
