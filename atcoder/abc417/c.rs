#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        arr: [i64; n]
    }

    // j - i = A[i] + A[j]
    // j - A[j] = A[i] + i

    let mut cnt = HashMap::new();
    let mut ans = 0;
    for j in 0..n {
        ans += *cnt.get(&(j as i64 - arr[j])).unwrap_or(&0) as i64;
        *cnt.entry(j as i64 + arr[j]).or_insert(0) += 1;
    }
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
