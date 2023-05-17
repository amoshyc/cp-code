#![allow(unused)]

fn main() {
    let sieve = SieveOfEratosthenes::new(3_000_000);
    let t = read::<usize>();
    for _ in 0..t {
        let a = read::<u64>();
        let mut ok = false;
        for &p in sieve.primes.iter() {
            if a % (p * p) == 0 {
                let q = a / (p * p);
                println!("{} {}", p, q);
                ok = true;
                break;
            }
        }
        if ok {
            continue;
        }
        for &q in sieve.primes.iter() {
            if a % q == 0 {
                let p = ((a / q) as f64).sqrt() as u64;
                println!("{} {}", p, q);
            }
        }
    }
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
