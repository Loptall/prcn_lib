use std::ops::{Add, Div};

//
pub fn binary_search<T, F>(l: T, r: T, pred: F) -> T
where
    T: Add<Output = T> + Div<Output = T> + PartialEq + From<u8> + Copy,
    F: Fn(T) -> bool,
{
    let mut l = l;
    let mut r = r;
    let two = T::from(2u8);

    loop {
        let m = (l + r) / two;
        if l == m || r == m {
            break l;
        }
        if pred(m) {
            l = m;
        } else {
            r = m;
        }
    }
}

#[test]
fn binary_search_test() {
    let v = vec![1, 3, 3, 3, 4, 6, 7, 7, 8, 10];
    // println!("{}", -1 / 2);
    assert_eq!(4, binary_search(v.len() as _, -1, |i| v[i as usize] > 3));
    assert_eq!(1, binary_search(v.len() as _, -1, |i| v[i as usize] >= 3));
}

/// vの要素について最初に現れたval以上の要素のインデックスを返す
pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] < *val) + 1) as usize
}

#[test]
fn lower_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(lower_bound(v, &0), 0);
    assert_eq!(lower_bound(v, &1), 0);
    assert_eq!(lower_bound(v, &2), 1);
    assert_eq!(lower_bound(v, &3), 1);
    assert_eq!(lower_bound(v, &4), 3);
    assert_eq!(lower_bound(v, &5), 4);
    assert_eq!(lower_bound(v, &999), 5);
}

/// vの要素について最初に現れたvalより大きい要素のインデックスを返す
pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
    (binary_search(-1i64, v.len() as i64, |i| v[i as usize] <= *val) + 1) as usize
}

#[test]
fn upper_bound_test() {
    let v: &[i32] = &[1, 3, 3, 4, 5];
    assert_eq!(upper_bound(v, &0), 0);
    assert_eq!(upper_bound(v, &1), 1);
    assert_eq!(upper_bound(v, &2), 1);
    assert_eq!(upper_bound(v, &3), 3);
    assert_eq!(upper_bound(v, &4), 4);
    assert_eq!(upper_bound(v, &5), 5);
    assert_eq!(upper_bound(v, &999), 5);
}
