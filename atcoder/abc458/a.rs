#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    let ans = (n..s.len() - n).map(|i| s[i]).collect::<Vec<char>>();
    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
