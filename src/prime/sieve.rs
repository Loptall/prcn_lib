use cargo_snippet::snippet;

/// 初期化の際にだけエラストテネスの篩を使って素数のリストを生成
///
/// `O(n)`
#[snippet("sieve")]
pub struct Sieve {
    size: usize,
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
        let mut spf = vec![None; n + 1];
        let mut is_prime = vec![true; n + 1];
        let mut primes = Vec::new();

        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..n + 1 {
            if is_prime[i] {
                primes.push(i);
                spf[i] = Some(i);
            }

            for prime in &primes {
                if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                    break;
                }

                is_prime[i * prime] = false;
                spf[i * prime] = Some(*prime);
            }
        }

        Self {
            size: n,
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
    let ps = sieve.primes((n as f64).sqrt().ceil() as usize);

    dbg!(&ps);

    for p in ps {
        let mut c = 0usize;
        while n % p == 0 {
            n /= p;
            c += 1;
        }
        if c != 0 {
            res.push((p, c));
        }
    }

    // `n`が素数だった
    if n > 1 {
        res.push((n, 1));
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
