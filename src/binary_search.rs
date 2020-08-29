use cargo_snippet::snippet;

pub use binary_serach::*;
#[snippet("binary_search", prefix = "pub use binary_search::*;")]
mod binary_serach {
    use std::ops::{Add, Div};

    /// # Usage
    ///
    /// `T`は基本的にusizeが入る想定
    ///
    /// `pred`について`l`側が`true`、
    /// `r`側が`false`を返すような単調性を持っているとき、
    ///
    /// `pred`を満たす`T`の中で最も大きいものを求める。
    ///
    ///
    /// # Graphic
    ///
    /// `pred = |x| x > 4` のとき
    ///
    /// 10 \# <- true
    ///
    /// 09 \## <- true
    ///
    /// 08 \### <- true
    ///
    /// 07 \#### <- true
    ///
    /// 06 \##### <- true
    ///
    /// 05 \###### <- true  ---  last element which apply `pred` !
    ///
    /// 04 \####### <- false
    ///
    /// 03 \######## <- false
    ///
    /// 02 \######### <- false
    ///
    /// 01 \########## <- false
    ///
    /// ### pred(l) == false のとき, 適合する要素がない => None
    ///
    /// ### pred(r) == true のとき, 見える範囲での最後の値 (r + 1 も満たすかもしれないけれど) => Some(r)
    ///
    /// 最初に`l`と`r`について`pred`がそれぞれ`true`、`false`を返すかどうかを調べるので、
    /// 範囲は閉空間で指定して良い。
    ///
    /// 返り値`res`は必ず、
    ///
    /// Some(inner) (l <= inner <= r) | None
    ///
    /// になる
    ///
    /// ```rust
    /// use sfcpl::binary_search::binary_search;
    ///
    /// let v = vec![1, 3, 3, 3, 4, 6, 7, 7, 8, 10];
    /// assert_eq!(3, binary_search(|i| v[i] < 4, 0, v.len() - 1).unwrap());
    /// assert_eq!(9, binary_search(|i| v[i] <= 10, 0, v.len() - 1).unwrap());
    /// ```
    pub fn binary_search<T, F>(pred: F, l: T, r: T) -> Option<T>
    where
        T: Add<Output = T> + Div<Output = T> + PartialEq + Copy + From<u8>,
        F: Fn(T) -> bool,
    {
        if !pred(l) {
            return None;
        }
        if pred(r) {
            return Some(r);
        }

        let mut l = l;
        let mut r = r;
        let two = T::from(2);

        loop {
            let m = (l + r) / two;
            if l == m {
                break Some(l);
            }

            if pred(m) {
                l = m;
            } else {
                r = m;
            }
        }
    }

    #[test]
    fn binary_search_test() {
        let v = vec![1, 3, 3, 3, 4, 6, 7, 7, 8, 10];

        assert_eq!(3, binary_search(|i| v[i] < 4, 0, v.len() - 1).unwrap());
        assert_eq!(9, binary_search(|i| v[i] <= 10, 0, v.len() - 1).unwrap());
    }

    /// `v`以上の要素が最初に現れるindex
    ///
    /// ```rust
    /// use sfcpl::binary_search::lower_bound;
    ///
    /// let v: &[i32] = &[1, 3, 3, 4, 5];
    /// assert_eq!(lower_bound(v, &0), 0);
    /// assert_eq!(lower_bound(v, &1), 0);
    /// assert_eq!(lower_bound(v, &2), 1);
    /// assert_eq!(lower_bound(v, &3), 1);
    /// assert_eq!(lower_bound(v, &4), 3);
    /// assert_eq!(lower_bound(v, &5), 4);
    /// assert_eq!(lower_bound(v, &999), 5);
    /// ```
    pub fn lower_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
        let t = binary_search(|x| v[x] < *val, 0, v.len() - 1);
        match t {
            Some(e) => e + 1,
            None => 0,
        }
    }

    #[test]
    fn lower_bound_test() {
        let v: &[i32] = &[1, 3, 3, 4, 5];
        assert_eq!(lower_bound(v, &0), 0);
        assert_eq!(lower_bound(v, &1), 0);
        assert_eq!(lower_bound(v, &2), 1);
        assert_eq!(lower_bound(v, &3), 1);
        assert_eq!(lower_bound(v, &4), 3);
        assert_eq!(lower_bound(v, &5), 4);
        assert_eq!(lower_bound(v, &999), 5);
    }

    /// `v`より大きい要素が最初に現れるindex
    ///
    /// ```rust
    /// use sfcpl::binary_search::upper_bound;
    ///
    /// let v: &[i32] = &[1, 3, 3, 4, 5];
    /// assert_eq!(upper_bound(v, &0), 0);
    /// assert_eq!(upper_bound(v, &1), 1);
    /// assert_eq!(upper_bound(v, &2), 1);
    /// assert_eq!(upper_bound(v, &3), 3);
    /// assert_eq!(upper_bound(v, &4), 4);
    /// assert_eq!(upper_bound(v, &5), 5);
    /// assert_eq!(upper_bound(v, &999), 5);
    /// ```
    pub fn upper_bound<T: PartialOrd>(v: &[T], val: &T) -> usize {
        let t = binary_search(|x| v[x] <= *val, 0, v.len() - 1);
        match t {
            Some(e) => e + 1,
            None => 0,
        }
    }

    #[test]
    fn upper_bound_test() {
        let v: &[i32] = &[1, 3, 3, 4, 5];
        assert_eq!(upper_bound(v, &0), 0);
        assert_eq!(upper_bound(v, &1), 1);
        assert_eq!(upper_bound(v, &2), 1);
        assert_eq!(upper_bound(v, &3), 3);
        assert_eq!(upper_bound(v, &4), 4);
        assert_eq!(upper_bound(v, &5), 5);
        assert_eq!(upper_bound(v, &999), 5);
    }
}
