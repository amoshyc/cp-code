#![allow(unused)]

use std::collections::HashMap;

const MOD: u64 = 998244353;

fn main() {
    let s = reads();
    let n = s.len();

    // dp[i, j] = number of string from s[..=i] that has sum j
    // if s[i] == (, then dp[i, j] -> dp[i + 1, j + 1]
    // if s[i] == ), then dp[i, j] -> dp[i + 1, j - 1]
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for i in 0..n {
        let mut new = vec![0u64; n + 1];
        for j in 0..=n {
            if (s[i] == '(' || s[i] == '?') && j + 1 < n {
                new[j + 1] += dp[j];
                new[j + 1] %= MOD;
            }
            if (s[i] == ')' || s[i] == '?') && j >= 1 {
                new[j - 1] += dp[j];
                new[j - 1] %= MOD;
            }
        }
        dp = new;
    }

    println!("{}", dp[0]);
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
