use cargo_snippet::snippet;

pub use sieve::*;
#[snippet(name = "sieve", prefix = "pub use sieve::*;")]
pub mod sieve {
    use std::collections::BTreeMap;
    /// 初期化の際にだけエラストテネスの篩を使って素数のリストを生成
    ///
    /// `O(n)`
    pub struct Sieve {
        size: usize,
        /// sieve[i] => i が振り落とされた数
        spf: Vec<usize>,
        /// `(0..=n)`までの範囲で
        /// `is_prime[i]` => `i`は素数
        is_prime: Vec<bool>,
        /// `(0..=n)`の全ての素数
        primes: Vec<usize>,
    }

    impl Sieve {
        /// 初期化、サイズが重要
        pub fn new(n: usize) -> Self {
            let mut spf: Vec<usize> = (0..=n).collect();
            let mut is_prime = vec![true; n + 1];

            is_prime[0] = false;
            is_prime[1] = false;

            for i in (2..).take_while(|&x| x * x <= n) {
                if spf[i] == i {
                    for j in (i * i..=n).step_by(i) {
                        if spf[j] == j {
                            is_prime[j] = false;
                            spf[j] = i;
                        }
                    }
                }
            }

            let primes = is_prime
                .iter()
                .enumerate()
                .filter(|x| *x.1)
                .map(|x| x.0)
                .collect();

            Self {
                size: n,
                spf,
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
    pub fn factorizations_with_sieve(sieve: &Sieve, n: usize) -> BTreeMap<usize, usize> {
        // Panic
        assert!(sieve.size.pow(2) >= n);

        let mut res = BTreeMap::new();

        factorizations_with_sieve_inner(sieve, n, &mut res);

        res
    }

    fn factorizations_with_sieve_inner(sieve: &Sieve, n: usize, res: &mut BTreeMap<usize, usize>) {
        if n == 1 {
            return;
        }

        let d = sieve.spf[n];
        *res.entry(d).or_insert(0) += 1;
        factorizations_with_sieve_inner(sieve, n / d, res);
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
        use maplit::btreemap;
        let sieve = Sieve::new(10000007);
        let f = factorizations_with_sieve(&sieve, 12);
        assert_eq!(f, btreemap![2 => 2, 3 => 1]);
        let f = factorizations_with_sieve(&sieve, 107);
        assert_eq!(f, btreemap![107 => 1]);
        // let f = factorizations_with_sieve(&sieve, 1000000007);
        // assert_eq!(f, btreemap![1000000007 => 1]);
    }
}
