#![allow(unused)]

use std::collections::HashSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        s: Chars,
    }

    let sieve = SieveOfEratosthenes::new(10_000_000);

    for &p in &sieve.primes {
        let p = p.to_string().chars().collect::<Vec<_>>();
        if p.len() != s.len() {
            continue;
        }

        let mut maps = vec![' '; 26];
        let mut used = HashSet::new();
        let mut ok = true;
        for (&c, &d) in s.iter().zip(p.iter()) {
            let idx = c as usize - 'a' as usize;

            if maps[idx] == ' ' {
                if !used.contains(&d) {
                    maps[idx] = d;
                    used.insert(d);
                    continue;
                } else {
                    ok = false;
                    break;
                }
            } else {
                ok &= maps[idx] == d;
            }
        }

        if ok {
            println!("{}", join(&p, ""));
            return;
        }
    }

    println!("-1");
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
