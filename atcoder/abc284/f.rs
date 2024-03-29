#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let t = reads();
    let a: Vec<u64> = t.iter().map(|&c| c as u64).collect();
    let b: Vec<u64> = t.iter().rev().map(|&c| c as u64).collect();

    let h = PolyHasher::new(2 * n, 31, 1_000_000_007);
    let ha = h.hash(&a);
    let hb = h.hash(&b);

    for i in 0..=n {
        // S = rev(T[i..(i + n)])
        //   => S = revT[(n - i)..(2n - i)]
        // S[0..i] = T[0..i]
        //   => revT[(n - i)..n] = T[0..i]
        // S[i..n] = T[(n + i)..(2n)]
        //   => revT[n..(2n - i)] = T[(n + i)..(2n)]
        let ok1 = h.range(&hb, n - i, n) == h.range(&ha, 0, i);
        let ok2 = h.range(&hb, n, 2 * n - i) == h.range(&ha, n + i, 2 * n);
        if ok1 && ok2 {
            let s: String = t[i..i + n].iter().rev().collect();
            println!("{}", s);
            println!("{}", i);
            std::process::exit(0);
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
