#![allow(unused_imports)]

use prcn_lib::modint::def::{ComTable, ModInt};
use prcn_lib::prelude::*;
use prcn_lib::bitset::BitSet;



fn main() {
    input! {
        n: u64,
    }

    println!("{:?}", BitSet::iter_all(n));
}
