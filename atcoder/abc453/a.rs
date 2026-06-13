#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let p = s.iter().position(|&c| c != 'o').unwrap_or(n);
    println!("{}", join(&s[p..], ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
