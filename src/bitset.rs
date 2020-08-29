use cargo_snippet::snippet;

pub use bitset::*;

#[snippet("bitset", prefix = "pub use bitset::*;")]
pub mod bitset {
    pub type BitSet = Vec<bool>;

    /// bitの数を持つ
    #[derive(Debug, Copy, Clone)]
    pub struct BitSetsGen {
        size: usize,
    }

    impl BitSetsGen {
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

    impl IntoIterator for BitSetsGen {
        type Item = BitSet;
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
    #[derive(Copy, Clone, Debug)]
    pub struct IntoIterBitSet {
        size: usize,
        current: usize,
    }

    impl Iterator for IntoIterBitSet {
        type Item = BitSet;
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
        let mut bs = BitSetsGen::new(3).into_iter();

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

    pub trait BitsOps {
        fn new(n: usize) -> Self;
        fn count_zeros(&self) -> usize;
        fn count_ones(&self) -> usize;
        fn grow(&mut self, bits: usize);
        fn put(&mut self, bits: usize) -> bool;
        fn toggle(&mut self, bit: usize);
        fn set(&mut self, bit: usize, into: bool);
        fn shl(&mut self, rhs: usize);
        fn shr(&mut self, rhs: usize);
        fn format(&self) -> String;
    }

    impl BitsOps for BitSet {
        fn new(n: usize) -> Self {
            let mut b = 1;
            let mut res = Vec::new();
            while b <= n {
                res.push(b & n != 0);
                b <<= 1;
            }

            res.reverse();
            res
        }
        fn count_zeros(&self) -> usize {
            self.iter().filter(|x| !**x).count()
        }
        fn count_ones(&self) -> usize {
            self.iter().filter(|x| **x).count()
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
        fn format(&self) -> String {
            self.iter().map(|x| if *x { '1' } else { '0' }).collect()
        }
    }

    #[test]
    fn construct_from_int() {
        let a = 10;
        let b: BitSet = BitsOps::new(a);
        assert_eq!(b, vec![true, false, true, false]);

        let a = 16;
        let b: BitSet = BitsOps::new(a);
        assert_eq!(b, vec![true, false, false, false, false]);

        let a = 15;
        let mut b: BitSet = BitsOps::new(a);
        assert_eq!(b, vec![true; 4]);

        assert_eq!(b.count_zeros(), 0);

        b.grow(3);

        assert_eq!(b.count_zeros(), 3);

        b.set(1, true);

        assert_eq!(b, vec![false, true, false, true, true, true, true]);

        b.shr(2);

        assert_eq!(b, vec![false, false, false, true, false, true, true]);

        assert_eq!(b.format(), "0001011".to_string());
    }
}
