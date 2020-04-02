// from std
pub use std::cmp::{max, min};
pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

// from num-integer
// binomial(n, r)     => nCr
// n.gcd(&m)          => gcd(n, m)
// n.lcm(&m)          => lcm(n, m)
// a.extended_gcd(&b) => Extended(gcd(a, b), x, y, ()) // a * x + b * y = gcd(a, b) の唯一解
pub use num::integer::binomial;
pub use num::Integer;

// from num-bigint
pub use num::{BigInt, BigUint};

// from random
pub use rand::random;

// from proconio
pub use proconio::{fastout, input, marker::*};

// from itertools
pub use itertools::*;

// from maplit
pub use maplit::{btreemap, btreeset, hashmap, hashset};

// from crate
pub use crate::accumulate::Accumulate;
pub use crate::binary_search::{binary_search, lower_bound, upper_bound};
pub use crate::math::*;
pub use crate::modint::{ComTable, ModInt};
pub use crate::shuffle::shuffle_vec;
pub use crate::util::*;
pub use crate::grid::Grid;
pub use crate::idx::Idx2D;
