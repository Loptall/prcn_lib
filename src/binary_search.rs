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
    assert_eq!(3, binary_search(v.len() as _, -1, |i| v[i as usize] >= 3));
}
