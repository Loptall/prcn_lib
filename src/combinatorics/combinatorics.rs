//! this mod is for snippet

use cargo_snippet::snippet;

pub use combinatorics::*;

#[snippet(name = "combinatorics", prefix = "pub use combinatorics::*;")]
pub mod combinatorics {
    use num_integer::Integer;
    use num_traits::{
        identities::{One, Zero},
        NumAssignOps, NumOps,
    };
    use num_traits::{Num, Pow};
    use std::cmp::Ordering;
    use std::convert::TryInto;
    use std::num::NonZeroU32;
    use std::num::ParseIntError;
    use std::ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
    };

    /// n % m
    /// ただし答えが負になる場合は余分にmを足すことで一意な値を保証
    ///
    /// # Panic
    /// 異なるmod間での演算をattemptした時
    fn compensated_rem(n: i64, m: usize) -> i64 {
        match n % m as i64 {
            // あまりが非負ならそのまま
            r if r >= 0 => r,
            // あまりが負ならmodを足す
            r => r + m as i64,
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Modulo {
        Static(NonZeroU32),
        Dynamic,
    }

    impl Modulo {
        pub fn get(&self) -> Option<u32> {
            match self {
                Modulo::Static(nz) => Some(nz.get()),
                Modulo::Dynamic => None,
            }
        }
    }

    /// `ModInt -> PrimiteveInt` への暗黙のキャストは行わない!
    /// (get関数を提供するのでそれ使ってどうぞ)
    ///
    /// `PrimitiveInt -> ModInt` は許可する
    #[derive(Debug, Clone, Copy)]
    pub struct ModInt {
        num: i64,
        _modulo: Modulo,
    }

    impl Into<usize> for ModInt {
        fn into(self) -> usize {
            self.get() as usize
        }
    }

    pub trait IntoModInt: Copy {
        fn to_mint<M: TryInto<u32> + Copy>(self, modulo: M) -> ModInt;
    }

    macro_rules! impl_into_mint {
            ($($t:ty),*) => {
                $(
                    impl IntoModInt for $t {
                        fn to_mint<M: TryInto<u32> + Copy>(self, modulo: M) -> ModInt {
                            ModInt::new(self, modulo)
                        }
                    }
                )*
            };
        }

    impl_into_mint!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);

    impl PartialEq for ModInt {
        fn eq(&self, other: &Self) -> bool {
            if !check_mod_eq(self, other).1 {
                panic!("cannot compare these values because they have different modulo number",)
            }
            self.get() == other.num
        }
    }

    // #[snippet("modint")]
    // impl Eq for ModInt {}

    impl PartialOrd for ModInt {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if !check_mod_eq(self, other).1 {
                None
            } else {
                Some(self.get().cmp(&other.num))
            }
        }
    }

    // #[snippet("modint")]
    // impl Ord for ModInt {
    //     fn cmp(&self, other: &Self) -> Ordering {
    //         self.partial_cmp(other).unwrap()
    //     }
    // }

    fn check_mod_eq(a: &ModInt, b: &ModInt) -> (NonZeroU32, bool) {
        match (a._modulo, b._modulo) {
            (Modulo::Static(a), Modulo::Static(b)) => {
                if a == b {
                    (a, true)
                } else {
                    // safe becase 1 != 0, yeah
                    (unsafe { NonZeroU32::new_unchecked(1) }, false)
                }
            }
            (Modulo::Static(m), Modulo::Dynamic) | (Modulo::Dynamic, Modulo::Static(m)) => {
                (m, true)
            }
            (Modulo::Dynamic, Modulo::Dynamic) => (unsafe { NonZeroU32::new_unchecked(1) }, false),
        }
    }

    impl ModInt {
        /// always `_modulo > num >= 0 && _modulo >= 1`
        pub fn new<N: TryInto<i64>, M: TryInto<u32> + Copy>(n: N, m: M) -> Self {
            let m =
                NonZeroU32::new(m.try_into().ok().expect("modulo number may be wrong")).unwrap();
            let r = n
                .try_into()
                .ok()
                .expect("modulo number maybe over i64 range");
            let num = compensated_rem(r, m.get() as usize);
            Self {
                num,
                _modulo: Modulo::Static(m),
            }
        }

        /// get inner value
        pub fn get(&self) -> i64 {
            self.num
        }

        /// mod of modint
        ///
        /// # Panic
        /// if variant is Modulo::Dynamic
        pub fn get_mod(&self) -> usize {
            self._modulo.get().unwrap() as usize
        }

        /// return the power of self with mod, using binary powering method
        /// cannot use of Dynamic type mod Self
        fn pow_mod(&self, mut exp: usize) -> Self {
            let mut res = 1;
            let mut base = self.get() as usize;
            let m = self.get_mod();
            while exp > 0 {
                if exp & 1 != 0 {
                    res *= base;
                    res %= m;
                }
                base *= base;
                base %= m;
                exp >>= 1;
            }

            Self::new(res, self.get_mod())
        }

        /// `a / b == a * b^(-1)` となる `b^(-1)` を求める
        pub fn inv(&self) -> i64 {
            // let mut a = self.get();
            // let m = self.get_mod() as i64;
            // let mut b = self.get_mod() as i64;
            // let mut u = 1i64;
            // let mut v = 0i64;

            // while b != 0 {
            //     let t = a / b;
            //     a -= t * b;
            //     swap(&mut a, &mut b);
            //     u -= t * v;
            //     swap(&mut u, &mut v);
            // }

            // u %= m;
            // if u < 0 { u += m; }
            // u

            // impl with num_integar::Integar::extended_gcd ...
            let x = self.get().extended_gcd(&(self.get_mod() as i64)).x;
            compensated_rem(x, self.get_mod())
        }
    }

    impl Add<Self> for ModInt {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let c = check_mod_eq(&self, &rhs);
            if !c.1 {
                panic!("modulo between two instance is different!",)
            }

            let r = self.get() + rhs.num;
            Self {
                num: if r >= self.get_mod() as i64 {
                    r - c.0.get() as i64
                } else {
                    r
                },
                _modulo: Modulo::Static(c.0),
            }
        }
    }

    impl AddAssign<Self> for ModInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl Sub<Self> for ModInt {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let c = check_mod_eq(&self, &rhs);
            if !c.1 {
                panic!("modulo between two instance is different!",)
            }
            let num = compensated_rem(self.get() - rhs.get(), c.0.get() as usize);
            Self {
                num,
                _modulo: Modulo::Static(c.0),
            }
        }
    }

    impl SubAssign<Self> for ModInt {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl Mul<Self> for ModInt {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            let c = check_mod_eq(&self, &rhs);
            if !c.1 {
                panic!("modulo between two instance is different!",)
            }
            let num = compensated_rem(self.get() * rhs.get(), c.0.get() as usize);
            Self {
                num,
                _modulo: Modulo::Static(c.0),
            }
        }
    }

    impl MulAssign<Self> for ModInt {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }

    impl Div<Self> for ModInt {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            let c = check_mod_eq(&self, &rhs);
            if !c.1 {
                panic!("modulo between two instance is different!",)
            }
            Self {
                num: self.get() * rhs.inv() % c.0.get() as i64,
                _modulo: Modulo::Static(c.0),
            }
        }
    }

    impl DivAssign<Self> for ModInt {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }

    impl Rem for ModInt {
        type Output = Self;
        fn rem(self, rhs: Self) -> Self::Output {
            let c = check_mod_eq(&self, &rhs);
            if !c.1 {
                panic!("modulo between two instance is different!",)
            }
            Self {
                num: self.num % rhs.num,
                _modulo: Modulo::Static(c.0),
            }
        }
    }

    impl RemAssign for ModInt {
        fn rem_assign(&mut self, rhs: Self) {
            *self = *self % rhs
        }
    }

    impl Zero for ModInt {
        fn zero() -> Self {
            ModInt {
                num: 0,
                _modulo: Modulo::Dynamic,
            }
        }
        fn is_zero(&self) -> bool {
            self.num == 0
        }
    }

    impl One for ModInt {
        fn one() -> Self {
            ModInt {
                num: 1,
                _modulo: Modulo::Dynamic,
            }
        }
        fn is_one(&self) -> bool {
            self.num == 1
        }
    }

    impl Num for ModInt {
        type FromStrRadixErr = ParseIntError;
        fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            let num = str
                .chars()
                .rev()
                .enumerate()
                .map(|(i, b)| radix.pow(i as u32) as i64 * b.to_digit(radix).unwrap() as i64)
                .sum::<i64>();
            Ok(ModInt {
                num,
                _modulo: Modulo::Dynamic,
            })
        }
    }

    impl Pow<usize> for ModInt {
        type Output = Self;
        // fn pow(self, exp: u32) -> Self::Output {
        //     if exp == 0 {
        //         return T::one();
        //     }

        //     while exp & 1 == 0 {
        //         base = base.clone() * base;
        //         exp >>= 1;
        //     }
        //     if exp == 1 {
        //         return base;
        //     }

        //     let mut acc = base.clone();
        //     while exp > 1 {
        //         exp >>= 1;
        //         base = base.clone() * base;
        //         if exp & 1 == 1 {
        //             acc = acc * base.clone();
        //         }
        //     }
        //     acc
        // }

        fn pow(self, exp: usize) -> Self::Output {
            self.pow_mod(exp)
        }
    }

    impl Factoriable for ModInt {
        fn falling(self, take: usize) -> Self {
            let mut res = Self::one();
            let mut c = self;
            for _ in 0..take {
                res *= c;
                c -= Self::one();
            }
            res
        }
        fn rising(self, take: usize) -> Self {
            let mut res = Self::one();
            let mut c = self;
            for _ in 0..take {
                res *= c;
                c += 1;
            }
            res
        }
    }

    macro_rules! impl_ops_between_mint_and_primitive {
        ($($t:ty),*) => {
            $(
                impl Add<$t> for ModInt {
                    type Output = Self;
                    fn add(self, rhs: $t) -> Self::Output {
                        self + Self::new(rhs as i64, self.get_mod())
                    }
                }
                impl AddAssign<$t> for ModInt {
                    fn add_assign(&mut self, rhs: $t) {
                        *self = *self + rhs;
                    }
                }
                impl Sub<$t> for ModInt {
                    type Output = Self;
                    fn sub(self, rhs: $t) -> Self::Output {
                        self - Self::new(rhs as i64, self.get_mod())
                    }
                }
                impl SubAssign<$t> for ModInt {
                    fn sub_assign(&mut self, rhs: $t) {
                        *self = *self - rhs;
                    }
                }
                impl Mul<$t> for ModInt {
                    type Output = Self;
                    fn mul(self, rhs: $t) -> Self::Output {
                        self * Self::new(rhs as i64, self.get_mod())
                    }
                }
                impl MulAssign<$t> for ModInt {
                    fn mul_assign(&mut self, rhs: $t) {
                        *self = *self * rhs;
                    }
                }
                impl Div<$t> for ModInt {
                    type Output = Self;
                    fn div(self, rhs: $t) -> Self::Output {
                        self / Self::new(rhs as i64, self.get_mod())
                    }
                }
                impl DivAssign<$t> for ModInt {
                    fn div_assign(&mut self, rhs: $t) {
                        *self = *self / rhs;
                    }
                }
            )*
        };
    }

    impl_ops_between_mint_and_primitive!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);

    pub trait PartialBinomialCoefficient {
        fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt>;
    }

    pub trait BinomialCoefficient: PartialBinomialCoefficient {
        /// `n C k`
        fn binomial(&self, n: usize, k: usize) -> ModInt {
            self.partial_binomial(n, k).unwrap()
        }
    }

    /// Binomial Coefficient Table with DP
    /// 二項係数を`O(1)`で計算するためのテーブル
    ///
    /// factrial = [1, 1, 2, 6, 24, 120, ...],
    ///
    /// `1 <= k <= n <= 10^7` 程度
    pub struct BCTDP {
        _modulo: NonZeroU32,
        // `factorial[i]` = iの階乗
        factorial: Vec<ModInt>,
        // `inv[i]` = iの逆元
        inverse: Vec<ModInt>,
        // `factorial_inverse[i]` = iの階乗の逆元
        factorial_inverse: Vec<ModInt>,
    }

    impl BCTDP {
        /// 初期化
        ///
        /// DPを用いて `O(n log m)`
        /// 割り算を用いるので `log m` がつく
        ///
        /// `1 <= k <= n <= 10^7` 程度
        pub fn new(n: usize, modulo: usize) -> BCTDP {
            let mut factorial = vec![ModInt::new(1, modulo), ModInt::new(1, modulo)];
            factorial.reserve_exact(n);
            let mut inverse = vec![ModInt::new(0, modulo), ModInt::new(1, modulo)];
            inverse.reserve_exact(n);
            let mut factorial_inverse = vec![ModInt::new(1, modulo), ModInt::new(1, modulo)];
            factorial_inverse.reserve_exact(n);

            for i in 2..=n {
                factorial.push(factorial[i - 1] * i);
                inverse.push(modulo.to_mint(modulo) - inverse[modulo % i] * (modulo / i));
                factorial_inverse.push(factorial_inverse[i - 1] * inverse[i]);
            }

            Self {
                _modulo: NonZeroU32::new(modulo as u32).unwrap(),
                factorial,
                inverse,
                factorial_inverse,
            }
        }

        pub fn get_mod(&self) -> usize {
            self._modulo.get() as usize
        }

        pub fn factorial(&self, n: usize) -> ModInt {
            self.factorial[n]
        }

        pub fn factorial_inverse(&self, n: usize) -> ModInt {
            self.factorial_inverse[n]
        }

        /// `n` の mod self._modulo における逆元
        pub fn inv(&self, n: usize) -> ModInt {
            self.inverse[n]
        }
    }

    impl PartialBinomialCoefficient for BCTDP {
        fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt> {
            Some(if n < k {
                ModInt::zero()
            } else {
                self.factorial[n] * self.factorial_inverse[k] * self.factorial_inverse[n - k]
            })
        }
    }

    impl BinomialCoefficient for BCTDP {}

    /// `n` が固定値のときに有効
    /// `(n(固定値), mod, _[i] = n C i)`
    ///
    /// 初期化: `O(n)`
    ///
    /// `1 <= n <= 10^9 && 1 <= k <= 10^7` 程度
    pub struct BCTholdN(usize, NonZeroU32, Vec<ModInt>);

    impl BCTholdN {
        pub fn new(mut n: usize, m: usize) -> Self {
            let size = n;
            let mut c = vec![ModInt::new(1, m), ModInt::new(n, m)];
            c.reserve_exact(n + 1);
            for i in 2..=n {
                n -= 1;
                let prev = *c.last().unwrap();
                c.push(prev * n / i);
            }

            Self(size, NonZeroU32::new(m as u32).unwrap(), c)
        }
    }

    impl PartialBinomialCoefficient for BCTholdN {
        /// #Panic
        ///
        /// self.0 == _n でないとき
        fn partial_binomial(&self, _n: usize, k: usize) -> Option<ModInt> {
            if _n != self.0 {
                None
            } else {
                Some(self.2[k])
            }
        }
    }

    impl BinomialCoefficient for BCTholdN {}

    #[test]
    fn hold_n_test() {
        let tbl = BCTholdN::new(10, 1000000007);
        assert_eq!(tbl.partial_binomial(10, 2).unwrap().get(), 45);
        assert_eq!(tbl.partial_binomial(10, 10).unwrap().get(), 1);
    }

    /// `n, k` の2変数についての `n C k` の表を作る
    ///
    /// `1 <= k <= n <= 2000` 程度
    pub struct BCTSmallNK {
        n: usize,
        _modulo: NonZeroU32,
        dp: Vec<Vec<ModInt>>,
    }

    impl BCTSmallNK {
        pub fn new(n: usize, modulo: usize) -> Self {
            let mut dp = vec![vec![ModInt::new(0, modulo); n + 1]; n + 1];
            dp[0][0] = 1.to_mint(modulo);
            for i in 1..n {
                dp[i][0] = 1.to_mint(modulo);
                for j in 1..n {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                }
            }
            Self {
                n,
                _modulo: NonZeroU32::new(modulo as u32).unwrap(),
                dp,
            }
        }

        pub fn size(&self) -> usize {
            self.n
        }

        pub fn get_mod(&self) -> usize {
            self._modulo.get() as usize
        }
    }

    impl PartialBinomialCoefficient for BCTSmallNK {
        fn partial_binomial(&self, n: usize, k: usize) -> Option<ModInt> {
            if n > self.size() || k > self.size() {
                panic!("n or k is too large, compere to dp table!",)
            }
            Some(self.dp[n][k])
        }
    }

    impl BinomialCoefficient for BCTSmallNK {}

    pub trait Factoriable: Sized + NumOps + NumAssignOps + Copy + TryInto<usize> {
        fn falling(self, take: usize) -> Self;
        fn rising(self, take: usize) -> Self;
        fn factorial(self) -> Self {
            self.falling(self.try_into().ok().unwrap())
        }
    }

    macro_rules! impl_factorialbe {
        ($($t:ty),*) => {
            $(
                impl Factoriable for $t {
                    fn falling(self, take: usize) -> Self {
                        let mut res = Self::one();
                        let mut c = self;
                        for _ in 0..take {
                            res *= c;
                            c -= Self::one();
                        }
                        res
                    }
                    fn rising(self, take: usize) -> Self {
                        let mut res = Self::one();
                        let mut c = self;
                        for _ in 0..take {
                            res *= c;
                            c += 1;
                        }
                        res
                    }
                }
            )*
        };
    }

    impl_factorialbe!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);

    /// `n P k` を `O(k)` で
    ///
    /// 内部はfallingをラップしているだけ
    pub fn permutation<T: Factoriable>(n: T, k: usize) -> T {
        n.falling(k)
    }

    pub fn permutation_with_table(table: &BCTDP, n: usize, k: usize) -> ModInt {
        if k > n {
            ModInt::new(0, table.get_mod())
        } else {
            table.factorial(n) * table.factorial_inverse(n - k)
        }
    }

    pub fn combination(n: ModInt, k: usize) -> ModInt {
        if k > n.get() as usize {
            // panic!("n < k, where n in ModInt, k in usize, so cannot calculate n C k",)
        }
        permutation(n, k) / k.factorial()
    }

    pub fn combination_with_table<T: BinomialCoefficient>(table: &T, n: usize, k: usize) -> ModInt {
        table.binomial(n, k)
    }
}
