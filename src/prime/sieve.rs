use cargo_snippet::snippet;

/// 初期化の際にだけエラストテネスの篩を使って素数のリストを生成
///
/// `O(n)`
#[snippet("sieve")]
pub struct Sieve {
    size: usize,
    /// sieve[i] => i が振り落とされた数
    sieve: Vec<Option<usize>>,
    /// `(0..=n)`までの範囲で
    /// `is_prime[i]` => `i`は素数
    is_prime: Vec<bool>,
    /// `(0..=n)`の全ての素数
    primes: Vec<usize>,
}

#[snippet("sieve")]
impl Sieve {
    /// 初期化、サイズが重要
    pub fn new(n: usize) -> Self {
        let mut sieve = vec![None; n * n + 1];
        let mut is_prime = vec![false; n + 1];
        let mut primes = Vec::new();

        is_prime[0] = false;
        is_prime[1] = false;
        sieve[0] = None;
        sieve[1] = None;

        for i in 2..=n {
            if sieve[i].is_some() {
                continue;
            }

            is_prime[i] = true;
            primes.push(i);

            (1..).map(|x| x * i).take_while(|x| *x <= n * n).for_each(|x| {
                sieve[x] = Some(i);
            })
        }

        Self {
            size: n,
            sieve,
            is_prime,
            primes,
        }
    }

    /// 自分自身の素数リストの有効な範囲
    pub fn size(&self) -> usize {
        self.size
    }

    /// `n`以下の全ての素数のリストを作る
    pub fn primes(&self, n: usize) -> Vec<usize> {
        assert!(self.size() >= n);
        self.primes
            .iter()
            .take_while(|x| **x <= n)
            .cloned()
            .collect()
    }

    /// `n`が素数かどうか
    ///
    /// # Panic
    /// `n > self.size`でindexing panicする
    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }
}

/// Seiveテーブルを用いた素因数分解
///
/// `n`を素因数分解するためには最小で、
/// `√n`までのサイズの素数テーブルが必要
///
/// # Panic
/// `sieve`のサイズが`√n`未満で不十分な場合にpanicします
#[snippet("sieve")]
pub fn factorizations_with_sieve(sieve: &Sieve, mut n: usize) -> Vec<(usize, usize)> {
    // Panic
    assert!(sieve.size.pow(2) >= n);

    let mut res = Vec::new();

    while n != 1 {
        let d = sieve.sieve[n].unwrap();
        let mut exp = 0usize;
        while n % d == 0 {
            exp += 1;
            n /= d;
        }
        res.push((d, exp));
    }

    res
}

#[test]
fn sieve_test() {
    let sieve = Sieve::new(10);
    assert_eq!(vec![2, 3, 5, 7], sieve.primes(10));
    assert_eq!(sieve.is_prime[2], true);
    assert_eq!(sieve.is_prime[9], false);
}

#[test]
fn fact_test() {
    let sieve = Sieve::new(100000);
    let mut f = factorizations_with_sieve(&sieve, 12);
    f.sort();
    assert_eq!(f, vec![(2, 2), (3, 1)]);
    let mut f = factorizations_with_sieve(&sieve, 107);
    f.sort();
    assert_eq!(f, vec![(107, 1)]);
    let mut f = factorizations_with_sieve(&sieve, 1000000007);
    f.sort();
    assert_eq!(f, vec![(1000000007, 1)]);
}
