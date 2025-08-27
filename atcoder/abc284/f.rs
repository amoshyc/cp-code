#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        t: Chars,
    }

    let mut r = t.clone();
    r.reverse();

    let hasher = PolyHasher2::new(t.len());
    let ht = hasher.hash(&t);
    let hr = hasher.hash(&r);
    // let ht = hasher.hash(&t.iter().map(|&c| c as u64).collect::<Vec<u64>>());
    // let hr = hasher.hash(&r.iter().map(|&c| c as u64).collect::<Vec<u64>>());

    // T = S[..i] + rev(S) + S[i..]
    // =>
    //     1. S[..i] = T[..i]
    //     2. S = rev(T[i..i + n]) = revT[2n - (i + n)..2n - i] = revT[n - i..2n - i]
    //     3. S[i..] = T[i + n..]
    // Substitude 2. into 1.:
    //     revT[n - i..2n - i][..i] = T[..i]
    //     revT[n - i..n] = T[..i]
    // Substitude 2. into 3.:
    //     revT[n - i..2n - i][i..] = T[i + n..]
    //     revT[n..2n - i] = T[i + n..]

    for i in 0..n {
        let ok1 = hasher.range(&hr, n - i, n) == hasher.range(&ht, 0, i);
        let ok2 = hasher.range(&hr, n, 2 * n - i) == hasher.range(&ht, i + n, 2 * n);
        if ok1 && ok2 {
            let mut s = t[i..i + n].to_vec();
            s.reverse();
            println!("{}", join(&s, ""));
            println!("{}", i);
            return;
        }
    }

    println!("-1");
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

struct PolyHasher2 {
    hasher0: PolyHasher,
    hasher1: PolyHasher,
}

impl PolyHasher2 {
    fn new(n: usize) -> Self {
        Self {
            hasher0: PolyHasher::new(n, 31, 1_000_000_007),
            hasher1: PolyHasher::new(n, 37, 1_000_000_009),
        }
    }

    fn hash(&self, arr: &[char]) -> (Vec<u64>, Vec<u64>) {
        let pref0 = self.hasher0.hash(arr);
        let pref1 = self.hasher1.hash(arr);
        (pref0, pref1)
    }

    // l..r
    // rev(S[l..r]) = revS[(n - r)..(n - l)]
    fn range(&self, pref: &(Vec<u64>, Vec<u64>), l: usize, r: usize) -> (u64, u64) {
        (
            self.hasher0.range(&pref.0, l, r),
            self.hasher1.range(&pref.1, l, r),
        )
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
