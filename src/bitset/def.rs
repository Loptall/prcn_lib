
use std::ops::{Shl, ShlAssign, Shr, ShrAssign, Index, IndexMut, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

#[derive(Debug, Clone, PartialEq, PartialOrd,)]
pub struct BitSet(Vec<bool>);

impl BitSet {
    pub fn set(length: usize) -> Self {
        BitSet(vec![false; length])
    }
}

impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, n: usize) -> &bool {
        &self.0[self.0.len() - n - 1]
    }
}

impl IndexMut<usize> for BitSet {
    fn index_mut(&mut self, n: usize) -> &mut bool {
        let length = self.0.len();
        &mut self.0[length - n - 1]
    }
}

impl Shr<usize> for BitSet {
    type Output = Self;
    fn shr(self, n: usize) -> Self {
        let ret = vec![false; n].iter().chain(self.0[..self.0.len() - n].iter()).copied().collect();
        Self(ret)
    }
}

impl ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, other: usize) {
        let copy = self.clone();
        *self = copy >> other;
    }
}

impl Shl<usize> for BitSet {
    type Output = Self;
    fn shl(self, n: usize) -> Self {
        let ret = self.0[self.0.len() - n..].iter().chain(vec![false; n].iter()).copied().collect();
        Self(ret)
    }
}

impl ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, other: usize) {
        let copy = self.clone();
        *self = copy << other;
    }
}

impl BitAnd for BitSet {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        let mut ret = vec![false; (self.0.len() as isize - other.0.len() as isize).abs() as usize];
        let mut sf = self.0;
        let mut ot = other.0;
        if sf.len() > ot.len() {
            sf = sf[sf.len() - ot.len()..].to_vec();
        } else {
            ot = ot[ot.len() - sf.len()..].to_vec();
        }
        for (i, j) in sf.iter().zip(ot.iter()) {
            ret.push(i & j);
        }
        Self(ret)
    }
}

impl BitAndAssign for BitSet {
    fn bitand_assign(&mut self, other: Self) {
        let copy = self.clone();
        *self = copy & other;
    }
}

impl BitOr for BitSet {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        let sf = self.clone();
        let ot = other.clone();
        let mut ret = if self.0.len() > other.0.len() {
            &sf.0[..sf.0.len() - ot.0.len()].to_vec()
        } else {
            &ot.0[..ot.0.len() - sf.0.len()].to_vec()
        };
        let mut sf = self.0;
        let mut ot = other.0;
        if sf.len() > ot.len() {
            sf = sf[sf.len() - ot.len()..].to_vec();
        } else {
            ot = ot[ot.len() - sf.len()..].to_vec();
        }
        for (i, j) in sf.iter().zip(ot.iter()) {
            ret.push(i | j);
        }
        Self(ret.to_vec())
    }
}

#[test]
fn test_idx() {
    let mut bs = BitSet::set(4);
    assert_eq!(bs[0], false);
    bs[0] = true;
    assert_eq!(bs[0], true);
}

#[test]
fn test_and() {
    let a = BitSet(vec![false, true, true, false, true, true]);
    let b = BitSet(vec![true, true, false, false]);
    assert_eq!(a & b, BitSet(vec![false, false, true, false, false, false]))
}