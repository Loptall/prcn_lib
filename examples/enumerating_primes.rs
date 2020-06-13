use cargo_snippet::snippet;
use std::io::stdin;
use std::str::FromStr;

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

pub fn input_three<T: FromStr, U: FromStr, V: FromStr>() -> (T, U, V) {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let s: Vec<&str> = buf.split_whitespace().collect();
    (
        s[0].parse::<T>().ok().unwrap(),
        s[1].parse::<U>().ok().unwrap(),
        s[2].parse::<V>().ok().unwrap(),
    )
}

fn main() {
    let (n, a, b): (usize, usize, usize) = input_three();
    let sieve = Sieve::new(n);
    let pi = sieve.primes(100).len();
    let mut ans = Vec::new();
    for p in sieve.primes(n).iter().skip(a).step_by(b) {
        ans.push(p.to_string());
    }

    println!("{} {}", pi, ans.len());
    for i in &ans[..ans.len() - 1] {
        print!("{} ", i);
    }
    println!("{}", ans[ans.len() - 1]);
}
