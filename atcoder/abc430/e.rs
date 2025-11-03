#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        tc: usize,
    }

    let hasher = PolyHasher::new(1_000_001, 31, 1_000_000_009);

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            a: Chars,
            b: Chars,
        }

        let n = a.len();
        let pref_a = hasher.hash(&a);
        let pref_b = hasher.hash(&b);

        let ok = |i: usize| -> bool {
            // rotate a i times
            // a[0..i], a[i..n] -> a[i..n], a[0..i]
            let s = hasher.range(&pref_a, 0, i);
            let t = hasher.range(&pref_a, i, n);
            let concat = (s * hasher.powr[n - i] % hasher.p + t) % hasher.p;
            concat == pref_b[n - 1]
        };

        if let Some(i) = (0..n).find(|&i| ok(i)) {
            ans.push(i as i64);
        } else {
            ans.push(-1);
        }
    }

    println!("{}", join(&ans, "\n"));
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

fn foldv<T: Clone>(arr: &[T], op: impl Fn(T, T) -> T) -> Vec<T> {
    let mut res = vec![arr[0].clone(); arr.len()];
    for i in 1..arr.len() {
        res[i] = op(res[i - 1].clone(), arr[i].clone());
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
        let powr = foldv(&vec![1; n], |acc, _| (acc * b) % p);
        let inv = powmod(powr[n - 1], p - 2, p);
        let pinv = foldv(&vec![inv; n], |acc, _| (acc * b) % p);
        let pinv = pinv.into_iter().rev().collect();
        Self { p, powr, pinv }
    }

    fn hash(&self, s: &[char]) -> Vec<u64> {
        let h = (0..s.len())
            .map(|i| (s[i] as u64) % self.p * self.powr[i] % self.p)
            .collect::<Vec<_>>();
        let pref = foldv(&h, |acc, x| (acc + x) % self.p);
        pref
    }

    // l..r
    // rev(S[l..r]) = revS[(n - r)..(n - l)]
    fn range(&self, pref: &[u64], l: usize, r: usize) -> u64 {
        assert!(l < pref.len());
        assert!(r <= pref.len());
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
