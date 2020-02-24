use prcn_lib::modint::*;
use prcn_lib::prelude::*;
use prcn_lib::prelude::input;


fn main() {
    let mut a = def::ModInt::new(100000006);
    let mut b = def::ModInt::new(10);

    a *= b;
    println!("{}", a);
}