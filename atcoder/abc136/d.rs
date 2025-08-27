#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let k = 2 * n;
    let mut dp = vec![vec![0; n]; 40];
    for i in 0..n {
        if s[i] == 'L' {
            dp[0][i] = i - 1;
        } else {
            dp[0][i] = i + 1;
        }
    }
    for k in 1..40 {
        for i in 0..n {
            dp[k][i] = dp[k - 1][dp[k - 1][i]];
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let mut x = i;
        for j in 0..40 {
            if (k >> j) & 1 == 1 {
                x = dp[j][x];
            }
        }
        ans[x] += 1;
    }

    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
