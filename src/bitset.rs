
use super::math::tools;

#[derive(Debug, Clone, PartialEq)]
pub struct BitSet(Vec<bool>);

impl BitSet {
    pub fn from_int(n: u64) -> Self {
        Self(bin_vec(n))
    }

    pub fn iter_all(n: u64) -> Vec<Self> {
        let mut begin = Self(vec![false; bin_scale(n)]);
        let end = Self::from_int(n);
        let mut ret = vec![begin.clone()];
        while begin.0 != end.0 {
            if begin.right() {
                for i in begin.0.iter_mut().rev().take_while(|x| **x) {
                    *i = !*i;
                }
                for i in begin.0.iter_mut().rev() {
                    if !*i {
                        *i = !*i;
                    }
                }
            } else {
                *begin.0.last_mut().unwrap() = true;
            }
            ret.push(begin.clone());
        }
        ret
    }

    pub fn right(&self) -> bool {
        *self.0.last().unwrap()
    }
}

use std::cmp::Ordering;

#[allow(irrefutable_let_patterns)]
impl PartialOrd for BitSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (s, o) in self.0.iter().rev().zip(other.0.iter().rev()) {
            match s.cmp(&o) {
                Ordering::Equal => continue,
                odr => return Some(odr),
            }
        }
        if let odr =  self.0.len().cmp(&other.0.len()) {
            Some(odr)
        } else {
            unreachable!()
        }
    }
}


/// `u64`の2進法での桁数
pub fn bin_scale(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    let mut n = n;
    while n >= 1 {
        n /= 2;
        count += 1;
    }
    count as usize
}

/// `u64`を桁ごとに`Vec<u64>`に分解
pub fn bin_vec(n: u64) -> Vec<bool> {
    let mut n = n;
    let mut ret = vec![false];
    while n > 0 {
        ret.push(n % 2 != 0);
        n >>= 1;
    }
    ret.reverse();
    ret
}