use std::collections::HashMap;

fn main() {
    let inp = readv::<u64>();
    let (n, p) = (inp[0], inp[1]);

    let mut dp = HashMap::<u64, u64>::new();
    println!("{}", e(&mut dp, n, p, 998244353));
}

fn e(dp: &mut HashMap<u64, u64>, s: u64, p: u64, m: u64) -> u64 {
    if dp.contains_key(&s) {
        return dp[&s];
    }
    if s <= 0 {
        dp.insert(s, 0);
    } else {
        let inv100 = powmod(100, m - 2, m);
        let term1 = e(dp, s.saturating_sub(2), p, m) * p % m * inv100 % m;
        let term2 = e(dp, s.saturating_sub(1), p, m) * (100 - p) % m * inv100 % m;
        dp.insert(s, (1 + term1 + term2) % m);
    }
    dp[&s]
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
