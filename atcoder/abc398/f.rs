#![allow(unused)]

fn main() {
    let s = reads();
    let n = s.len();
    let mut r = s.clone();
    r.reverse();

    let arr_s = mapv(&s, |c| *c as u64);
    let arr_r = mapv(&r, |c| *c as u64);

    let hasher = PolyHasher2::new(n);
    let hs = hasher.hash(&arr_s);
    let hr = hasher.hash(&arr_r);

    for i in 0..n {
        // is_palidrome(S[i..])
        // S[i..] == rev(S[i..])
        // S[i..] == revS[0..n - i]
        if hasher.range(&hs, i, n) == hasher.range(&hr, 0, n - i) {
            println!("{}{}", join(&s, ""), join(&r[n - i..], ""));
            return;
        }
    }
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

struct PolyHasher2 {
    prime: (u64, u64),
    powr: Vec<(u64, u64)>,
    pinv: Vec<(u64, u64)>,
}

impl PolyHasher2 {
    fn new(n: usize) -> Self {
        let base = (31, 37);
        let prime = (1_000_000_007, 1_000_000_009);
        let mut powr = vec![(1, 1); n];
        let mut pinv = vec![(1, 1); n];
        for i in 1..n {
            powr[i].0 = powr[i - 1].0 * base.0 % prime.0;
            powr[i].1 = powr[i - 1].1 * base.1 % prime.1;
        }
        pinv[n - 1].0 = powmod(powr[n - 1].0, prime.0 - 2, prime.0);
        pinv[n - 1].1 = powmod(powr[n - 1].1, prime.1 - 2, prime.1);
        for i in (0..(n - 1)).rev() {
            pinv[i].0 = pinv[i + 1].0 * base.0 % prime.0;
            pinv[i].1 = pinv[i + 1].1 * base.1 % prime.1;
        }
        Self { prime, powr, pinv }
    }

    fn hash(&self, arr: &[u64]) -> Vec<(u64, u64)> {
        assert!(arr.iter().all(|&x| x != 0));
        let mut h = vec![(0, 0); arr.len()];
        h[0].0 = arr[0] % self.prime.0;
        h[0].1 = arr[0] % self.prime.1;
        for i in 1..arr.len() {
            h[i].0 = (h[i - 1].0 + arr[i] * self.powr[i].0 % self.prime.0) % self.prime.0;
            h[i].1 = (h[i - 1].1 + arr[i] * self.powr[i].1 % self.prime.1) % self.prime.1;
        }
        h
    }

    // l..r
    // rev(S[l..r]) = revS[(n - r)..(n - l)]
    fn range(&self, h: &[(u64, u64)], l: usize, r: usize) -> (u64, u64) {
        assert!(l < h.len());
        assert!(r <= h.len());
        if l == r {
            (0, 0)
        } else if l == 0 {
            h[r - 1]
        } else {
            let h0 = (self.prime.0 + h[r - 1].0 - h[l - 1].0) % self.prime.0 * self.pinv[l].0;
            let h1 = (self.prime.1 + h[r - 1].1 - h[l - 1].1) % self.prime.1 * self.pinv[l].1;
            (h0 % self.prime.0, h1 % self.prime.1)
        }
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
