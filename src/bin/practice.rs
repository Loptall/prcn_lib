use prcn_lib::modint::*;
use prcn_lib::prelude::input;
use prcn_lib::prelude::*;

fn main() {
    let mut a = def::ModInt::new(100000006);
    let mut b = def::ModInt::new(10);

    a *= b;
    println!("{}", a);
}