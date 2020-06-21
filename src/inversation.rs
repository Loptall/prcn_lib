use crate::monoid::Monoid;
use crate::segment_tree::SegmentTree;

#[derive(Debug, Copy, Clone)]
struct Sum(usize);

impl Monoid for Sum {
    fn identity() -> Self {
        Sum(0usize)
    }
    fn op(x: &Self, y: &Self) -> Self {
        Self(x.0 + y.0)
    }
}

impl Into<Sum> for usize {
    fn into(self) -> Sum {
        Sum(self)
    }
}

pub fn inversation(v: &Vec<usize>) -> usize {
    let ma = *v.iter().max().unwrap();
    let mut v = v.clone();
    v.dedup();
    let mut s = SegmentTree::<Sum>::new(&vec![0usize; ma]);
    let mut t = 0usize;
    for i in 0..v.len() {
        t += i - s.range(0, v[i]).0;
        s.update(v[i] - 1, Sum(s.get_raw(v[i] - 1).unwrap().0 + 1));
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
