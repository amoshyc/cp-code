#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        arr: [Chars; n],
    }

    let max_len = arr.iter().map(|s| s.len()).max().unwrap();
    let hasher = PolyHasher::new(max_len, 31, 1_000_000_007);

    let mut cnt = HashMap::new();
    let mut ans = 0;
    for i in 0..n {
        let pref = hasher.hash(&arr[i]);
        for j in 0..pref.len() {
            let v = *cnt.get(&pref[j]).unwrap_or(&0) as i64;
            ans += v;
            cnt.insert(pref[j], v + 1);
        }
    }

    println!("{ans}");
}

// a ^ b % m
fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut base = a;
    while b > 0 {
        if b & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    res
}

fn foldv(arr: &[u64], op: impl Fn(u64, u64) -> u64) -> Vec<u64> {
    let mut res = vec![arr[0]; arr.len()];
    for i in 1..arr.len() {
        res[i] = op(res[i - 1], arr[i]);
    }
    res
}

struct PolyHasher {
    p: u64,
    powr: Vec<u64>,
    pinv: Vec<u64>,
}

impl PolyHasher {
    fn new(n: usize, b: u64, p: u64) -> Self {
        let powr = foldv(&vec![1; n], |acc, x| acc * b % p);
        let inv = powmod(powr[n - 1], p - 2, p);
        let pinv = foldv(&vec![inv; n], |acc, x| acc * b % p);
        let pinv = pinv.into_iter().rev().collect();
        Self { p, powr, pinv }
    }

    fn hash(&self, arr: &[char]) -> Vec<u64> {
        let arr = (0..arr.len())
            .map(|i| arr[i] as u64 * self.powr[i] % self.p)
            .collect::<Vec<_>>();
        let pref = foldv(&arr, |acc, x| acc + x);
        pref
    }

    // l..r
    fn range(&self, pref: &[u64], l: usize, r: usize) -> u64 {
        if l == r {
            0
        } else if l == 0 {
            pref[r - 1]
        } else {
            (self.p + pref[r - 1] - pref[l - 1]) % self.p * self.pinv[l] % self.p
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
