#![allow(unused)]

fn main() {
    let n = read::<u64>();
    let sieve = SieveOfEratosthenes::new(1_000_000);
    let p = sieve.primes.clone();

    let mut ans = 0;
    for i in (0..).take_while(|&i| p[i] * p[i] * p[i] * p[i] * p[i] <= n) {
        for j in (i + 1..).take_while(|&j| p[i] * p[i] * p[j] * p[j] * p[j] <= n) {
            let ub = ((n / (p[i] * p[i] * p[j])) as f64).sqrt() as u64;
            let k = partition_point(&p, |&x| x <= ub);
            if k - 1 > j {
                ans += k - j - 1;
            }
        }
    }
    println!("{}", ans);
}

// arr.partition_point is added at 1.52.0
// 1 1 1 0 0 0
//       ^
fn partition_point<T, P: FnMut(&T) -> bool>(arr: &[T], mut pred: P) -> usize {
    arr.binary_search_by(|x| {
        if pred(x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
}

struct SieveOfEratosthenes {
    primes: Vec<u64>,
}

impl SieveOfEratosthenes {
    fn new(max_val: usize) -> SieveOfEratosthenes {
        let mut is_prime = vec![true; max_val + 1];
        let mut primes = vec![];
        for i in 2..=max_val {
            if is_prime[i] {
                primes.push(i as u64);
                let mut j = i * i;
                while j <= max_val {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        SieveOfEratosthenes { primes }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
