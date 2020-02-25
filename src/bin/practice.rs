#![allow(unused_imports)]

use prcn_lib::modint::def::{ComTable, ModInt};
use prcn_lib::prelude::*;

fn main() {
    input! {
        n: u64, k: u64
    }

    let table = ComTable::new();
    let ans = table.combination(n, k);
    println!("{}", ans);
}
