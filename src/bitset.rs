#[derive(Debug, Copy, Clone)]
pub struct BitSet {
    len: usize,
    set: usize,
}

impl BitSet {
    pub fn new(n: usize) -> Self {
        Self { len: n, set: 0 }
    }

    pub fn as_bools(&self) -> Vec<bool> {
        let mut res = Vec::with_capacity(self.len);
        for p in (0..self.len).rev() {
            res.push(1 << p & self.set != 0)
        }
        res
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for BitSet {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.set == 2usize.pow(self.len() as u32) - 1 {
            None
        } else {
            let res = *self;
            self.set += 1;
            Some(res.as_bools())
        }
    }
}

#[test]
fn bs_iter() {
    let mut bs = BitSet::new(3);

    assert!(bs.next().unwrap() == vec![false, false, false]);
    assert!(bs.next().unwrap() == vec![false, false, true]);
    assert!(bs.next().unwrap() == vec![false, true, false]);
    assert!(bs.next().unwrap() == vec![false, true, true]);
    assert!(bs.next().unwrap() == vec![true, false, false]);

    // panic!()
}
