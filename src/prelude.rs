// input!, Chars等 のみを提供
pub use proconio::{input, marker::*};

// 内部クレートより使用頻度の多い機能を提供
pub use crate::{math::{max, min}, io::{exit, yes_no}};