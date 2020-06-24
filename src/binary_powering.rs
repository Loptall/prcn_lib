use cargo_snippet::snippet;

/// 二分累乗法
/// `O(log n)`で累乗を求める
///
/// modを取らないときはmに0を指定
#[snippet]
pub fn binary_powering(n: usize, r: usize, m: usize) -> usize {
    let mut a = n;
    let mut n = r;
    let mut res = 1usize;
    let m = if m != 0 { Some(m) } else { None };

    while n > 0 {
        if n & 1 != 0 {
            res *= a;
            if let Some(m) = m {
                res %= m
            }
        }
        a *= a;
        if let Some(m) = m {
            a %= m;
        }
        n >>= 1;
    }
    res
}

#[test]
fn pow_test() {
    assert_eq!(9, binary_powering(3, 2, 10000));
    assert_eq!(1024, binary_powering(2, 10, 0));
    assert_eq!(1, binary_powering(100, 0, 0));
    assert_eq!(1, binary_powering(10, 2, 3))
}
