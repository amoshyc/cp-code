#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [i64; n],
    }

    // dp[i, 0/1] = maximum total of while verting A[i - 1] and A[i] or not
    let mut dp = vec![vec![0, 0]; n];
    dp[1][0] = arr[0] + arr[1];
    dp[1][1] = -(arr[0] + arr[1]);
    for i in 2..n {
        // don't invert
        dp[i][0] = (dp[i - 1][0] + arr[i]).max(dp[i - 1][1] + arr[i]);
        // invert
        let from0 = dp[i - 1][0] - arr[i - 1] - arr[i - 1] - arr[i];
        let from1 = dp[i - 1][1] - arr[i] + arr[i - 1] + arr[i - 1];
        dp[i][1] = from0.max(from1);
    }

    println!("{}", dp[n - 1][0].max(dp[n - 1][1]));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
