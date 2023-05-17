#![allow(unused)]

fn main() {
    let hr1 = PolyHasher::new(500_000, 31, 1_000_000_007);
    let hr2 = PolyHasher::new(500_000, 29, 1_000_000_009);

    let n = read::<usize>();
    let mut h1 = vec![vec![]; n];
    let mut h2 = vec![vec![]; n];
    for i in 0..n {
        let s = reads();
        let a = s.iter().map(|&c| c as u64).collect::<Vec<u64>>();
        h1[i] = hr1.hash(&a);
        h2[i] = hr2.hash(&a);
    }

    let mut cnt = std::collections::HashMap::new();
    for i in 0..n {
        for j in 0..=h1[i].len() {
            let key = (hr1.range(&h1[i], 0, j), hr2.range(&h2[i], 0, j));
            *cnt.entry(key).or_insert(0) += 1;
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        let mut lb = 0;
        let mut ub = h1[i].len() + 1;
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            let key = (hr1.range(&h1[i], 0, m), hr2.range(&h2[i], 0, m));
            if cnt[&key] >= 2 {
                lb = m;
            } else {
                ub = m;
            }
        }
        ans.push(lb);
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
    // S[l..r] = revS[(n - 1 - r)..(n - 1 - l)]
    fn range(&self, h: &[u64], l: usize, r: usize) -> u64 {
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
