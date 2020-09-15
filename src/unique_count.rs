use cargo_snippet::snippet;

use std::collections::BTreeMap;

/// 配列に含まれる要素(T: Ord + Copy)別にその個数を数えて
/// `BTreeMap<T, usize>`の形にして返す
#[snippet(name = "unique_count")]
pub fn unique_count<T: Ord + Copy>(v: &[T]) -> BTreeMap<T, usize> {
    let mut map = BTreeMap::new();
    for e in v {
        let h = map.entry(*e).or_insert(0);
        *h += 1;
    }

    map
}

#[test]
fn count_test() {
    let v = vec![1, 2, 2, 3, 3, 3, 4, 5, 7];
    let map = unique_count(&v);
    assert_eq!(
        map,
        maplit::btreemap![1 => 1, 2 => 2, 3 => 3, 4 => 1, 5 => 1, 7 => 1]
    );
}
