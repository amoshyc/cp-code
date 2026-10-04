#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = vec![m / n; n];
    for i in 0..(m % n) {
        ans[i] += 1;
    }
    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
