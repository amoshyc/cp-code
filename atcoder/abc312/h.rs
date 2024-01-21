#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let p1 = 1_000_000_000 + 7;
    let p2 = 1_000_000_000 + 9;
    let hasher1 = PolyHasher::new(200_001, 29, p1);
    let hasher2 = PolyHasher::new(200_001, 31, p2);

    let mut ans = vec![!0; n];
    let mut set = HashSet::new();
    let mut mem: HashMap<(u64, u64), (i32, u64, u64)> = HashMap::new();

    for i in 0..n {
        let s = mapv(&reads(), |c| *c as u64);
        let m = s.len();
        let h1 = hasher1.hash(&s)[m - 1];
        let h2 = hasher2.hash(&s)[m - 1];

        let mut k = 1;
        let mut r1 = h1; // hash of repeated s
        let mut r2 = h2; // hash of repeated s
        if mem.contains_key(&(h1, h2)) {
            k = mem[&(h1, h2)].0;
            r1 = mem[&(h1, h2)].1;
            r2 = mem[&(h1, h2)].2;
        }

        while set.contains(&(r1, r2)) {
            k += 1;
            r1 = (r1 * hasher1.powr[m] % p1 + h1) % p1;
            r2 = (r2 * hasher2.powr[m] % p2 + h2) % p2;
        }
        set.insert((r1, r2));

        mem.insert((h1, h2), (k, r1, r2));
        
        ans[i] = k;
    }

    println!("{}", join(&ans, " "));
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

struct PolyHasher {
    prime: u64,
    powr: Vec<u64>,
    pinv: Vec<u64>,
}

impl PolyHasher {
    fn new(n: usize, base: u64, prime: u64) -> PolyHasher {
        let mut powr = vec![1; n];
        let mut pinv = vec![1; n];
        for i in 1..n {
            powr[i] = powr[i - 1] * base % prime;
        }
        pinv[n - 1] = powmod(powr[n - 1], prime - 2, prime);
        for i in (0..(n - 1)).rev() {
            pinv[i] = pinv[i + 1] * base % prime;
        }
        PolyHasher { prime, powr, pinv }
    }

    fn hash(&self, arr: &[u64]) -> Vec<u64> {
        assert!(arr.iter().all(|&x| x != 0));
        let mut h = vec![0; arr.len()];
        h[0] = arr[0] % self.prime;
        for i in 1..arr.len() {
            h[i] = (h[i - 1] + arr[i] * self.powr[i] % self.prime) % self.prime;
        }
        h
    }

    // l..r
    // S[l..r] = revS[(n - r)..(n - l)]
    fn range(&self, h: &[u64], l: usize, r: usize) -> u64 {
        assert!(l < h.len());
        assert!(r <= h.len());
        if l == r {
            0
        } else if l == 0 {
            h[r - 1]
        } else {
            (self.prime + h[r - 1] - h[l - 1]) % self.prime * self.pinv[l] % self.prime
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
