#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, i64); n],
    }

    let mut sum = vec![0; m + 1];
    let mut cnt = vec![0; m + 1];
    for (a, b) in ab {
        sum[a] += b;
        cnt[a] += 1;
    }

    let ans: Vec<_> = (1..=m)
        .map(|i| sum[i] as f64 / cnt[i] as f64)
        .map(|x| format!("{:.6}", x))
        .collect();
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
