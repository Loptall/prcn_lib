//! Varifeid

/// `i`が素数 <=> `self.0[i]`
#[derive(Debug)]
pub struct Prime(Vec<bool>);

impl Prime {
    /// エラストテネスの篩を用いて`n`より小さい素数を調べます
    /// 大体`O(n log log n)`なはず
    ///
    /// AtCoder Language Test 202001 コードテスト上で
    /// 10^8が限界 <- 十分
    pub fn init(n: usize) -> Self {
        let mut sieve = vec![true; n];
        sieve[0] = false;
        sieve[1] = false;

        let mut i = 2;
        while i * i <= n {
            if sieve[i] {
                (2..)
                    .map(|x| x * i)
                    .take_while(|x| *x < n)
                    .for_each(|x| sieve[x] = false);
            }
            i += 1;
        }

        Self(sieve)
    }

    /// 前計算によるテーブルを用いて素因数分解をします
    ///
    /// 返り値は`Vec<(素因数, 指数)>`の形式
    ///
    /// 十分に大きいテーブルを渡してください
    ///
    /// # Panic
    /// `self`の最大値が`n`より大きいときパニックする
    ///
    /// ```
    /// let table = Prime::init(10);
    /// assert_eq!(table.factorization(9), vec![(3, 2)]); // ok!
    /// // assert_eq!(table.factorization(10), vec![(2, 1), (5, 1)]); // panic! Because table.len() <= n
    /// ```
    pub fn factorization(&self, n: usize) -> Vec<(usize, usize)> {
        assert!(self.0.len() > n);

        let mut res = Vec::new();
        let ps = self.primes();
        let mut n = n;
        for p in ps {
            let mut count = 0;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            if count > 0 {
                res.push((p, count));
            }
        }
        if n > 1 {
            res.push((n, 1));
        }
        res
    }

    pub fn primes(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|t| *t.1)
            .map(|x| x.0)
            .collect()
    }

    /// `n`が素数かどうかをテーブルを用いて判定します
    ///
    /// `O(1)`
    ///
    /// # Panic
    /// `n >= self.0.len()`のとき、panicします。
    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n < self.0.len());

        self.0[n]
    }
}

#[test]
fn sieve_test() {
    let p = Prime::init(10);
    assert_eq!(
        &[false, false, true, true, false, true, false, true, false, false],
        &p.0[..]
    );
}

#[test]
fn fact_test() {
    let table = Prime::init(10);
    assert_eq!(table.factorization(9), vec![(3, 2)]); // ok!
                                                      // assert_eq!(table.factorization(10), vec![(2, 1), (5, 1)]); // panic! Because table.len() <= n
}

/// 素因数分解をします。
///
/// 呼び出すごとに新しく篩を作るので
/// 複数回を呼び出すときは計算量が増大します
/// 初期化したPrime構造体に対してメゾットを呼んでください。
/// (その代わりにパニックしません)
pub fn factorization(n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let ps = Prime::init(n).primes();
    let mut n = n;
    for p in ps {
        let mut count = 0;
        while n % p == 0 {
            n /= p;
            count += 1;
        }
        if count > 0 {
            res.push((p, count));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

/// Find the first factor (other than 1) of a number
fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };

    for n in (3..).step_by(2).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }

    x
}

/// 試し割りによる素数判定です
///
/// `O(√n)`
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    firstfac(n) == n
}
