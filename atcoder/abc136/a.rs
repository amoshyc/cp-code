#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        a: usize, b: usize, c: usize,
    }

    let ans = c.saturating_sub(a - b);
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
