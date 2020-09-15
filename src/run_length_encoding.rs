use cargo_snippet::snippet;

/// ランレングス圧縮
#[snippet(name = "run_length_encoding")]
pub fn run_length_encoding<T: PartialEq + Copy>(v: &[T]) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    let mut c = 1usize;
    let mut h = v[0];
    for e in v.iter().skip(1) {
        if h != *e {
            res.push((h, c));
            c = 1;
            h = *e;
        } else {
            c += 1;
        }
    }
    res.push((h, c));

    res
}

#[test]
fn rle_test() {
    let v = "abbccc";
    let e = run_length_encoding(&v.chars().collect::<Vec<_>>().as_slice());
    assert_eq!(e, vec![('a', 1), ('b', 2), ('c', 3)]);

    let v = vec![1, 1, 2, 2, 1, 1, 2, 2];
    let e = run_length_encoding(&v);
    assert_eq!(e, vec![(1, 2), (2, 2), (1, 2), (2, 2)]);
}
