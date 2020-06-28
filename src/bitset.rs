use cargo_snippet::snippet;

/// bitの数を持つ
#[snippet("bitset")]
#[derive(Debug, Copy, Clone)]
pub struct BitSet {
    size: usize,
}

#[snippet("bitset")]
impl BitSet {
    /// サイズを指定してコンストラクト
    pub fn new(n: usize) -> Self {
        Self { size: n }
    }

    /// サイズを返す
    pub fn len(&self) -> usize {
        self.size
    }

    /// 合計で生成される組み合わせの数、`(2 ^ size)`
    pub fn combinations(&self) -> usize {
        1 << self.size
    }
}

#[snippet("bitset")]
impl IntoIterator for BitSet {
    type Item = Vec<bool>;
    type IntoIter = IntoIterBitSet;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterBitSet {
            size: self.len(),
            current: 0,
        }
    }
}

/// BitSetの所有権を奪ったIterator
///
///(実際にはコピートレイトにより`into_iter()`後もBitSetは有効である)
///
/// 保持するのはサイズと次に生成する数字だけ
///
/// Iteratorは長さ`2 ^ size`で、
/// はじめは`[false, false, ..., false, false]`から始まり、
///
/// `[false, false, ..., false, true]`
///
/// `[false, false, ..., true, false]`
///
/// `[false, false, ..., true, true]`のように右詰で続く。
///
/// 最後は`[true, true, ..., true, true]`である。
#[snippet("bitset")]
#[derive(Copy, Clone, Debug)]
pub struct IntoIterBitSet {
    size: usize,
    current: usize,
}

#[snippet("bitset")]
impl Iterator for IntoIterBitSet {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == (1 << self.size) {
            None
        } else {
            let res = Some(
                (0..self.size)
                    .rev()
                    .map(|x| self.current & (1 << x) != 0)
                    .collect::<Vec<_>>(),
            );
            self.current += 1;
            res
        }
    }
}

#[test]
fn bs_iter() {
    let mut bs = BitSet::new(3).into_iter();

    assert_eq!(Some(vec![false, false, false]), bs.next());
    assert_eq!(Some(vec![false, false, true]), bs.next());
    assert_eq!(Some(vec![false, true, false]), bs.next());
    assert_eq!(Some(vec![false, true, true]), bs.next());
    assert_eq!(Some(vec![true, false, false]), bs.next());
    assert_eq!(Some(vec![true, false, true]), bs.next());
    assert_eq!(Some(vec![true, true, false]), bs.next());
    assert_eq!(Some(vec![true, true, true]), bs.next());
    assert_eq!(None, bs.next());
}

#[snippet("bitset")]
pub trait BitsOps {
    fn count_zeros(&self) -> usize;
    fn count_ones(&self) -> usize;
    fn grow(&mut self, bits: usize);
    fn put(&mut self, bits: usize) -> bool;
    fn toggle(&mut self, bit: usize);
    fn set(&mut self, bit: usize, into: bool);
    fn shl(&mut self, rhs: usize);
    fn shr(&mut self, rhs: usize);
}

#[snippet("bitset")]
impl BitsOps for Vec<bool> {
    fn count_zeros(&self) -> usize {
        self.iter().filter(|x| **x).count()
    }
    fn count_ones(&self) -> usize {
        self.iter().filter(|x| !**x).count()
    }
    fn grow(&mut self, bits: usize) {
        self.reverse();
        self.append(&mut vec![false; bits]);
        self.reverse();
    }
    fn put(&mut self, bits: usize) -> bool {
        let prev = self[bits];
        self[bits] = true;
        prev
    }
    fn toggle(&mut self, bit: usize) {
        self[bit] = !self[bit];
    }
    fn set(&mut self, bit: usize, into: bool) {
        self[bit] = into;
    }
    fn shl(&mut self, rhs: usize) {
        self.append(&mut vec![false; rhs]);
    }
    fn shr(&mut self, rhs: usize) {
        let len = self.len();
        let mut res = vec![false; rhs];
        res.append(&mut self[..len - rhs].to_vec());
        *self = res;
    }
}
