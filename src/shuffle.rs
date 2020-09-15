use cargo_snippet::snippet;
use rand::{thread_rng, Rng};

#[snippet(name = "shuffle")]
pub fn shuffle<T>(v: &mut Vec<T>) {
    for i in (1..v.len()).rev() {
        v.swap(thread_rng().gen_range(0, i), i);
    }
}

#[test]
fn test_shuffle() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    shuffle(&mut v);
    println!("{:?}", v);
}
