#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s[0] == s[s.len() - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
