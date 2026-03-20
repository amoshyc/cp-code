#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    s[0] = s[0].to_ascii_lowercase();
    println!("Of{}", join(&s, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
