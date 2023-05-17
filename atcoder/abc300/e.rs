#![allow(unused)]

use std::collections::HashMap;

// f(x > N) = 0
// f(x = N) = 1
// f(x < N) = 1/6 * (f(x * d) for d in 1..=6)
// f(x < N) = 1/5 * (f(x * d) for d in 2..=6)
fn f(x: u64, n: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if dp.contains_key(&x) {
        return dp[&x];
    }

    if x == n {
        return 1;
    }

    let mut prob = 0;
    for d in 2..=6 {
        if d * x <= n {
            prob += f(d * x, n, dp);
            prob %= 998_244_353;
        }
    }
    prob *= powmod(5, 998_244_353 - 2, 998_244_353);
    prob %= 998_244_353;

    dp.insert(x, prob);
    prob
}

fn main() {
    let n = read::<u64>();
    let m = 998_244_353;
    let mut dp = HashMap::new();
    println!("{}", f(1, n, &mut dp));
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
