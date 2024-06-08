#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let t = reads();

    let tt = mapv(&t, |&c| c as u64);
    let mut r = tt.clone();
    r.reverse();

    let hasher = PolyHasher::new(t.len(), 31, 1_000_000_007);
    let ht = hasher.hash(&tt);
    let hr = hasher.hash(&r);

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
    // rev(S[l..r]) = revS[(n - r)..(n - l)]
    fn range(&self, h: &[u64], l: usize, r: usize) -> u64 {
        assert!(l < h.len());
        assert!(r <= h.len());
        if l == r {
            0
        } else if l == 0 {
            h[r - 1]
        } else {
            let ans = (self.prime + h[r - 1] - h[l - 1]) % self.prime * self.pinv[l] % self.prime;
            ans
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
