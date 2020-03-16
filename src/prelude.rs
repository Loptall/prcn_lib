// input!, Chars等 のみを提供
pub use proconio::{input, marker::*, fastout};

// 内部クレートより使用頻度の多い機能を提供
pub use crate::{
    io::{exit, yes_no},
    math::tools::{max, min},
    modint::def::ModInt,
};
