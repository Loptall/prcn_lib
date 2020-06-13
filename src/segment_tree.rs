//! Varified

//! # Usage

//! monoid_def!(Mm<i32>, std::i32::MAX, |x, y| { min(x, y) });
//! monoid_def!(Max<usize>, std::usize::MIN, |x, y| { max(x, y) });

//! #[test]
//! fn debug() {
//!     let mut s =
//!         SegmentTree::<Max>::new(&[Max(1usize), Max(2), Max(3), Max(2), Max(1), Max(3), Max(1)]);
//!     dbg!(&s);

//!     s.update(7, Max(100));
//!     dbg!(&s);
//! }

//! #[test]
//! fn get() {
//!     let s = SegmentTree::<Max>::new(&[Max(1usize), Max(2), Max(3), Max(2), Max(1), Max(3), Max(1)]);
//!     dbg!(&s);

//!     assert_eq!(s.range(0, 2).0, 2);
//!     assert_eq!(s.range(3, 6).0, 3);
//!     assert_eq!(s.range(0, 8).0, 3);
//! }

use crate::monoid::Monoid;

use cargo_snippet::snippet;

#[snippet(name = "segment_tree", include = "monoid")]
#[derive(Debug, Clone)]
pub struct SegmentTree<T: Monoid> {
    leaves: usize,
    size: usize,
    value: Vec<T>,
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
        let leaves = n.next_power_of_two();
        let size = 2 * leaves - 1;
        let mut value = vec![T::identity(); size];

        for i in (0..size).rev() {
            if i >= leaves - 1 {
                if i + 1 - leaves < n {
                    value[i] = (v[i + 1 - leaves]).into();
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

    /// `i`番目の葉をとる
    pub fn get_raw(&self, i: usize) -> Option<T> {
        if self.leaves <= i {
            panic!("too big index")
        }
        Some(self.value[self.size - i - 1])
    }

    /// `i`番目の葉を`v`で更新
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

    /// 区間、`[from..to)`を指定の`Monoid`でfoldした演算結果
    pub fn range(&self, from: usize, to: usize) -> T {
        self.range_inner(from, to, 0, self.leaves, 0)
    }

    fn range_inner(&self, from: usize, to: usize, l_bound: usize, r_bound: usize, k: usize) -> T {
        if from <= l_bound && to >= r_bound {
            self.value[k]
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

// #[snippet("monoid")]
// // #[macro_export]
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
