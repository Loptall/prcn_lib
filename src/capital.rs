
/// 愚直で間に合わない操作を境界をチェックして一括で行う
pub fn sub_bound(n: u64, m: u64) -> (u64, u64) {
    if n >= m {
        (n - m, 0)
    } else {
        (0, m - n)
    }
}



