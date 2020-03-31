//! 入出力周りを効率化したい

pub fn count_map<T: Eq + std::hash::Hash + Copy>(v: &[T]) -> std::collections::HashMap<T, usize> {
    let mut map = std::collections::HashMap::new();
    for e in v {
        let h = map.entry(*e).or_insert(0);
        *h += 1;
    }
    map
}

/// プログラムを終了
pub fn exit() {
    use std::process;
    process::exit(0)
}
