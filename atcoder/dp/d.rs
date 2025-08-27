#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, u64); n],
    }

    let ws = items.iter().map(|&(w, v)| w).collect::<Vec<usize>>();
    let vs = items.iter().map(|&(w, v)| v).collect::<Vec<u64>>();

    // dp[i, j] = maximum total value from items 0..=i while total weight is j
    // Consider using item i or not, dp[i, j] = max(dp[i - 1][j - w[i]] + v[i], dp[i - 1, j])

    let mut dp = vec![vec![0; w + 1]; n];
    dp[0][0] = 0;
    dp[0][ws[0]] = vs[0];
    for i in 1..n {
        for j in 0..=w {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            if j >= ws[i] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - ws[i]] + vs[i]);
            }
        }
    }

    println!("{}", dp[n - 1].iter().max().unwrap());
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
