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
        for p in 0..self.len {
            res.push(1 << p & self.set != 0)
        }
        res
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for BitSet {
    type Item = Self;
    fn next(&mut self) -> Option<Self::Item> {
        if self.set == 2usize.pow(self.len() as u32) - 1 {
            None
        } else {
            let res = Some(*self);
            self.set += 1;
            res
        }
    }
}

#[test]
fn bs_iter() {
    let mut bs = BitSet::new(3);

    assert!(bs.next().unwrap().set == 0);
    assert!(bs.next().unwrap().set == 1);
    assert!(bs.next().unwrap().set == 2);
    assert!(bs.next().unwrap().set == 3);
    assert!(bs.next().unwrap().set == 4);
    assert!(bs.next().unwrap().set == 5);
    assert!(bs.next().unwrap().set == 6);
    assert!(bs.next().unwrap().set == 7);

    // panic!()
}
