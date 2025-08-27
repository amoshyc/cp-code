#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    let ans = (a + b).max(a - b).max(a * b);
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
