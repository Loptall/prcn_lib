use num_traits::{NumAssignOps, NumOps, One};

use std::convert::TryInto;

use crate::modint::ModInt;

pub trait Factoriable: Sized + NumOps + NumAssignOps + Copy + TryInto<usize> {
    fn falling(self, take: usize) -> Self;
    fn rising(self, take: usize) -> Self;
    fn factorial(self) -> Self {
        self.falling(self.try_into().ok().unwrap())
    }
}

macro_rules! impl_factorialbe {
    ($($t:ty),*) => {
        $(
            impl Factoriable for $t {
                fn falling(self, take: usize) -> Self {
                    let mut res = Self::one();
                    let mut c = self;
                    for _ in 0..take {
                        res *= c;
                        c -= Self::one();
                    }
                    res
                }
                fn rising(self, take: usize) -> Self {
                    let mut res = Self::one();
                    let mut c = self;
                    for _ in 0..take {
                        res *= c;
                        c += 1;
                    }
                    res
                }
            }
        )*
    };
}

impl_factorialbe!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, ModInt);

#[test]
fn fact_test_prim() {
    let a = 7;
    assert_eq!(a.falling(3), 210);

    let a = 10;
    assert_eq!(a.rising(2), 110);

    let a = 5;
    assert_eq!(a.factorial(), 120);
}

#[test]
fn fact_test_mint() {
    let a = ModInt::new(7, 4);
    assert_eq!(a.falling(3).get(), 2);

    let a = ModInt::new(6, 7); // 720
    assert_eq!(a.factorial().get(), 6);
}
