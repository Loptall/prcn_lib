use cargo_snippet::snippet;

/// 二分累乗法
/// `O(log n)`で累乗を求める
///
/// modがいらないときはstd::usize::MAXで適当に
#[snippet]
pub fn binary_powering(n: usize, r: usize, m: usize) -> usize {
    let mut a = n;
    let mut n = r;
    let mut res = 1usize;
    while n > 0 {
        if n & 1 != 0 {
            res *= a;
            res %= m;
        }
        a *= a;
        a %= m;
        n >>= 1;
    }
    res
}
