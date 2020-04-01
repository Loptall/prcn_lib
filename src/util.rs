use std::collections::BTreeMap;

/// 配列に含まれる要素(T: Ord + Copy)別にその個数を数えて
/// `BTreeMap<T, usize>`の形にして返す
/// ```
/// use std::collections::BTreeMap;
/// use prcn_lib::util::count_element_to_map;
///
/// let v = vec![1, 2, 2, 3, 3, 3, 4, 5, 7];
/// let map = count_element_to_map(&v);
/// assert_eq!(
///     map,
///     [(1, 1), (2, 2), (3, 3), (4, 1), (5, 1), (7, 1)]
///         .into_iter()
///         .map(|t| (t.0, t.1))
///         .collect::<BTreeMap<i32, usize>>()
/// );
/// ```
pub fn count_element_to_map<T: Ord + Copy>(v: &[T]) -> BTreeMap<T, usize> {
    let mut map = BTreeMap::new();
    for e in v {
        let h = map.entry(*e).or_insert(0);
        *h += 1;
    }

    map
}

/// プログラムを終了
/// 単純に`std::process::exit(0)`の省略型
pub fn exit() {
    use std::process;
    process::exit(0)
}
