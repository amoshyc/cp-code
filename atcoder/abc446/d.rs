#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    // dp[i] = the length of the longest subseq ending with value i
    // dp[i] = max(1, dp[i], dp[i - 1] + 1)
    let mut dp = HashMap::new();
    for x in arr {
        let mut val = *dp.get(&x).unwrap_or(&0);
        val = val.max(dp.get(&(x - 1)).unwrap_or(&0) + 1);
        dp.insert(x, val);
    }

    let ans = dp.values().max().unwrap();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
