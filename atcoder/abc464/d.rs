#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        tc: usize,
    }

    let mut res = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            arr: Chars,
            xs: [i64; n],
            ys: [i64; n - 1],
        }

        // dp[i, 0/1] = maximum total happiness after day i while day i is changed to sunny/rainy
        let inf = 10i64.pow(18);
        let mut dp = vec![[-inf, -inf]; n];

        if arr[0] == 'S' {
            dp[0] = [0, -xs[0]];
        } else {
            dp[0] = [-xs[0], 0];
        }

        for i in 1..n {
            // dp[i][0]
            let cost = if arr[i] == 'S' { 0 } else { xs[i] };
            dp[i][0] = (dp[i - 1][0]).max(dp[i - 1][1] + ys[i - 1]) - cost;

            // dp[i][1]
            let cost = if arr[i] == 'R' { 0 } else { xs[i] };
            dp[i][1] = (dp[i - 1][1]).max(dp[i - 1][0]) - cost;
        }

        res.push(dp[n - 1][0].max(dp[n - 1][1]));
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
