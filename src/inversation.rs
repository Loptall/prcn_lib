use crate::structure::fenwick_tree::FenwickTree;

use cargo_snippet::snippet;

#[snippet("inversation", include = "fenwick_tree")]
/// 配列の転倒数をFenwickTree<Sum>を利用してカウント
///
/// `O(n log n)`
///
/// #[test]
/// fn inversation_test() {
///     let v = vec![3, 1, 2];
///     assert_eq!(inversation(&v), 2);
///     let v = vec![3, 2, 1];
///     assert_eq!(inversation(&v), 3);
///     let v = vec![3, 3, 3];
///     assert_eq!(inversation(&v), 0);
///     let v = vec![1, 2, 4, 3, 6, 5];
///     assert_eq!(inversation(&v), 2);
/// }
pub fn inversation(v: &Vec<usize>) -> usize {
    let m = *v.iter().max().unwrap();
    let mut f = FenwickTree::new(m + 1);
    let mut t = 0usize;
    for i in 0..v.len() {
        t += i - f.sum(v[i] + 1);
        f.add(v[i], 1);
    }
    t
}

#[test]
fn inversation_test() {
    let v = vec![3, 1, 2];
    assert_eq!(inversation(&v), 2);
    let v = vec![3, 2, 1];
    assert_eq!(inversation(&v), 3);
    let v = vec![3, 3, 3];
    assert_eq!(inversation(&v), 0);
    let v = vec![1, 2, 4, 3, 6, 5];
    assert_eq!(inversation(&v), 2);
}
