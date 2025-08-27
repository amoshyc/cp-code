#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    // dp[i] = minimum cost to reach stone i
    let mut dp = vec![10u64.pow(18); n];
    dp[0] = 0;
    for i in 1..n {
        if i >= 1 {
            dp[i] = dp[i].min(dp[i - 1] + h[i].abs_diff(h[i - 1]));
        }
        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + h[i].abs_diff(h[i - 2]));
        }
    }
    println!("{}", dp[n - 1]);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
