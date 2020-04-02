//! Varified

use rand::random;

pub fn shuffle_vec<T>(v: &mut Vec<T>) {
    let len = v.len();
    for i in 0..1000 {
        let a: usize = random::<usize>() % len;
        let b: usize = random::<usize>() % len;
        if a == b {
            continue;
        }
        if i % 2 == 0 {
            v[a.min(b)..=b.max(a)].rotate_left((a.max(b) - a.min(b)) / 2);
        } else {
            v[a.min(b)..=b.max(a)].rotate_right((a.max(b) - a.min(b)) / 2);
        }
    }
}

#[test]
fn test_shuffle() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    shuffle_vec(&mut v);
    println!("{:?}", v);
}
