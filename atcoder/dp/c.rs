#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [[u64; 3]; n],
    }

    // dp[i, 0/1/2] = maximum total points after i-th day and doing a/b/c at i-th day
    let mut dp = vec![vec![0, 0, 0]; n];
    dp[0] = arr[0].clone();
    for i in 1..n {
        for j in 0..3 {
            for k in 0..3 {
                if k != j {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + arr[i][j]);
                }
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
