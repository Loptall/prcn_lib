#![allow(unused_imports)]

use prcn_lib::modint::def::{ComTable, ModInt};
use prcn_lib::prelude::*;
use prcn_lib::bitset::def::BitSet;
use prcn_lib::graph::def::*;


fn main() {
    input! {
        a: f64
    }
    println!("{}", a as i64);
}
