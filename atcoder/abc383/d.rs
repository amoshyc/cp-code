#![allow(unused)]

fn main() {
    // if x has 9 divisors, x can only be one of following forms:
    // 1. x = p^8
    // 2. x = p^2 q^2

    // For case 1, simply inspect all the primes.
    // For case 2, wlog, assume (p >= q), inspect p and binary search the primes to find the number of q

    let n = read::<u64>();

    let sieve = SieveOfEratosthenes::new(2_000_000);
    let primes = sieve.primes;

    let mut ans = 0;

    // Case 1
    for p in primes.iter() {
        if p.saturating_pow(8) <= n {
            ans += 1;
        } else {
            break;
        }
    }

    // Case 2
    for (i, p) in primes.iter().enumerate() {
        let j = primes.partition_point(|q| (p * p).saturating_mul(q * q) <= n);
        ans += j.min(i);
        if j == 0 {
            break;
        }
    }

    println!("{}", ans);
}

struct SieveOfEratosthenes {
    primes: Vec<u64>,
}

impl SieveOfEratosthenes {
    fn new(v: usize) -> Self {
        let mut is_prime = vec![true; v + 1];
        let mut primes = vec![];
        for i in 2..=v {
            if is_prime[i] {
                primes.push(i as u64);
                for j in ((i * i)..=v).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        Self { primes }
    }

    fn factorize(&self, mut x: u64) -> Vec<(u64, u64)> {
        assert!(x > 1);
        let mut res = vec![];
        for &p in self.primes.iter() {
            let mut exp = 0;
            while x % p == 0 {
                exp += 1;
                x = x / p;
            }
            if exp > 0 {
                res.push((p, exp))
            }
            if p * p > x {
                break;
            }
        }
        if x > 1 {
            res.push((x, 1));
        }
        res
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
