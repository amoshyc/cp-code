#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        w: u64,
        items: [(u64, usize); n],
    }

    let ws = items.iter().map(|&(w, v)| w).collect::<Vec<u64>>();
    let vs = items.iter().map(|&(w, v)| v).collect::<Vec<usize>>();
    let v = vs.iter().sum::<usize>();

    // dp[i][j] = minimum total weight from items 0..=i while total value is j
    // Consider using item i or not, dp[i][j] = min(dp[i - 1][j - v[i]] + w[i], dp[i - 1][j])
    let inf = 10u64.pow(18);
    let mut dp = vec![vec![inf; v + 1]; n];
    dp[0][0] = 0;
    dp[0][vs[0]] = ws[0];
    for i in 1..n {
        for j in 0..=v {
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            if j >= vs[i] {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - vs[i]] + ws[i]);
            }
        }
    }

    let ans = (0..=v).filter(|&v| dp[n - 1][v] <= w).max().unwrap();
    println!("{}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
