#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();
    let ans = (0..(2 * n - 1))
        .map(|i| if i % 2 == 0 { s[i / 2] } else { 'o' })
        .collect::<Vec<_>>();
    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
