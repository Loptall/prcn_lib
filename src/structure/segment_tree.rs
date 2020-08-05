//! # Usage

//! monoid_def!(Mm<i32>, std::i32::MAX, |x, y| { min(x, y) });
//! monoid_def!(Max<usize>, std::usize::MIN, |x, y| { max(x, y) });

//! #[test]
//! fn debug() {
//!     let mut s =
//!         SegmentTree::<Max>::new(&[1usize, 2, 3, 2, 1, 3, 1]);
//!     dbg!(&s);

//!     s.update(7, 100);
//!     dbg!(&s);
//! }

//! #[test]
//! fn get() {
//!     let s = SegmentTree::<Max>::new(&[1usize, 2, 3, 2, 1, 3, 1]);
//!     dbg!(&s);

//!     assert_eq!(s.range(0, 2).0, 2);
//!     assert_eq!(s.range(3, 6).0, 3);
//!     assert_eq!(s.range(0, 8).0, 3);
//! }

use super::algebraic_traits::monoid::Monoid;

use cargo_snippet::snippet;

#[snippet(name = "segment_tree")]
#[derive(Debug, Clone)]
pub struct SegmentTree<T: Monoid> {
    len: usize,
    size: usize,
    segment: Vec<T>,
}

#[snippet("segment_tree")]
fn childrens_idx(n: usize) -> (usize, usize) {
    (n * 2 + 1, n * 2 + 2)
}

#[snippet("segment_tree")]
fn parent_idx(n: usize) -> usize {
    (n - 1) / 2
}

#[snippet("segment_tree")]
impl<T: Monoid + Clone + Copy> SegmentTree<T> {
    pub fn new<I: Into<T> + Copy>(v: &[I]) -> Self {
        let n = v.len();
        let len = n.next_power_of_two();
        let size = 2 * len - 1;
        let mut segment = vec![T::identity(); size];

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

        Self { len, size, segment }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn childrens(&self, n: usize) -> (T, T) {
        let (left, right) = childrens_idx(n);
        (self.segment[left], self.segment[right])
    }

    /// `i`番目の葉の参照をとる
    pub fn get(&self, i: usize) -> Option<&T> {
        self.segment.get(self.size() - self.len() + i)
    }

    /// `i`番目の葉の可変参照をとる
    ///
    /// # Unsafe
    ///
    /// 葉の部分は変更されうるけれど、その親要素へ変更が伝達されない
    ///
    /// update()を使うこと
    pub unsafe fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        let size = self.size();
        let len = self.len();
        self.segment.get_mut(size - len + i)
    }

    /// `i`番目の葉を`v`で更新
    pub fn update(&mut self, i: usize, v: T) {
        let mut cur = self.size() - self.len() + i;
        self.segment[cur] = v;
        loop {
            if cur == 0 {
                break;
            }
            cur = parent_idx(cur);
            let (left, right) = self.childrens(cur);
            self.segment[cur] = T::op(&left, &right);
        }
    }

    /// 区間、`[from..to)`を指定の`Monoid`でfoldした演算結果
    pub fn range(&self, from: usize, to: usize) -> T {
        self.range_inner(from, to, 0, self.len(), 0)
    }

    fn range_inner(&self, from: usize, to: usize, l_bound: usize, r_bound: usize, k: usize) -> T {
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

// macro_rules! monoid_def {
//     {
//         $M:ident<$t:ty>,
//         $id:expr,
//         $me:expr
//     } => {
//         #[derive(Debug, Clone, Copy)]
//         pub struct $M($t);

//         impl Monoid for $M {
//             fn identity() -> Self {
//                 $M($id)
//             }

//             fn op(x: &Self, y: &Self) -> Self {
//                 let f = $me;
//                 $M(f(x.0, y.0))
//             }
//         }

//         impl Into<$M> for $t {
//             fn into(self) -> $M {
//                 $M(self)
//             }
//         }
//     };
// }

// monoid_def! {
//     Max<usize>,
//     std::usize::MIN,
//     |a: usize, b: usize| a.max(b)
// }

// #[test]
// fn ux_test() {
//     let v = vec![1usize, 2, 3, 4, 5, 6, 7, 8];
//     let s = SegmentTree::<Max>::new(&v);
//     assert_eq!(s.range(0, 3).0, 3);
//     assert_eq!(s.range(2, 2).0, 0);
//     assert_eq!(s.range(0, 8).0, 8);
// }

// #[test]
// fn get_test() {
//     let s = SegmentTree::<Max>::new(&vec![1, 2, 3, 4]);
//     assert_eq!(s.get(1).unwrap().0, 2);
//     assert_eq!(s.get(3).unwrap().0, 4);
// }
