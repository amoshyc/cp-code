#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        tc: usize,
    }

    // LCM is not associative under MOD
    // LCM(LCM(a, b), c)
    // = LCM(MOD + 3, c)
    // != LCM(3, c)

    // We need to use prime factorization.

    // LCM(except arr[i]) = LCM(all) removing (the effect of arr[i])
    // The effect of arr[i] = check each p^e of arr[i] if it is part of LCM(all).
    // If it is, replace it with previous exponent.

    let sieve = SieveOfEratosthenes::new(10_000);
    let mut res = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            arr: [u64; n],
        }

        let mut pe = vec![vec![]; n];
        for i in 0..n {
            if arr[i] > 1 {
                pe[i] = sieve.factorize(arr[i]);
            }
        }

        let mut exps = HashMap::new();
        for i in 0..n {
            for &(p, e) in &pe[i] {
                exps.entry(p).or_insert(vec![]).push(e);
            }
        }
        for (p, es) in exps.iter_mut() {
            es.sort();
        }

        // overall lcm
        let mut lcm = 1;
        for (&p, es) in &exps {
            lcm = lcm * powmod(p, *es.last().unwrap(), MOD);
            lcm = lcm % MOD;
        }

        let mut ans = vec![lcm; n];
        for i in 0..n {
            for &(p, e) in &pe[i] {
                let es = exps.get(&p).unwrap();

                // if p^e is part of overall lcm
                if e == es[es.len() - 1] {
                    if es.len() == 1 || es[es.len() - 2] < e {
                        // replace p^e with previous exponent
                        ans[i] = ans[i] * powmod(p, e * (MOD - 2), MOD);
                        ans[i] = ans[i] % MOD;
                        if es.len() >= 2 {
                            ans[i] = ans[i] * powmod(p, es[es.len() - 2], MOD);
                            ans[i] = ans[i] % MOD;
                        }
                    }
                }
            }
        }

        res.push(join(&ans, " "));
    }

    println!("{}", join(&res, "\n"));
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut base = a % m;
    let mut res = 1;
    while b != 0 {
        if b & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    res
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
