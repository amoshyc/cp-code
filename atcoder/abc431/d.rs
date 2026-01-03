#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        whb: [(usize, i64, i64); n],
    }

    // dp[i, j] = maximum happiness when the body weight is j using items 0..=i
    // dp[i, j] = max of following:
    //      (attached to head) dp[i - 1, j] + h[i]
    //      (attached to body) dp[i - 1, j - w[i]] + b[i]

    let inf = 10i64.pow(18);

    let mut w = vec![0; n];
    let mut h = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        (w[i], h[i], b[i]) = whb[i];
    }

    let sum_w = w.iter().sum::<usize>();
    let mut dp = vec![-inf; sum_w + 1];
    dp[0] = h[0];
    dp[w[0]] = b[0];
    for i in 1..n {
        for j in (0..=sum_w).rev() {
            dp[j] = dp[j] + h[i];
            if j >= w[i] {
                dp[j] = dp[j].max(dp[j - w[i]] + b[i]);
            }
        }
    }

    let mut ans = 0;
    for body in 0..=sum_w {
        let head = sum_w - body;
        if head <= body {
            ans = ans.max(dp[body]);
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
