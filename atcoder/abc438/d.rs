#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr_a: [i64; n],
        arr_b: [i64; n],
        arr_c: [i64; n],
    }

    // dp[i, 0/1/2] = maximum total sum of arr[0..=i] if consider only head / head+body / head+body+tail
    let mut dp = vec![vec![0, 0, 0]; n];
    dp[0][0] = arr_a[0];
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] + arr_a[i];
        if i >= 1 {
            dp[i][1] = dp[i - 1][0].max(dp[i - 1][1]) + arr_b[i];
        }
        if i >= 2 {
            dp[i][2] = dp[i - 1][1].max(dp[i - 1][2]) + arr_c[i];
        }
    }

    println!("{}", dp[n - 1][2]);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
