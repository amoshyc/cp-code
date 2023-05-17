use std::collections::HashSet;

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    let _ = read::<usize>();
    let arr = readv::<u64>();

    if arr.iter().all(|&x| x == 1) {
        return "pairwise coprime".to_string();
    }

    let g = arr.iter().fold(0, |acc, &x| gcd(acc, x));
    if g != 1 {
        return "not coprime".to_string();
    }

    let val = arr.iter().max().unwrap();
    let sieve = SieveOfEratosthenes::new(*val as usize);
    let mut used_primes = HashSet::new();
    for a in arr {
        if a != 1 {
            for (p, _) in sieve.factorize(a) {
                if used_primes.contains(&p) {
                    return "setwise coprime".to_string();
                }
                used_primes.insert(p);
            }
        }
    }
    "pairwise coprime".to_string()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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