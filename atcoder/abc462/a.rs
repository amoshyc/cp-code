#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans: Vec<_> = s.iter().filter(|&c| c.is_digit(10)).collect();
    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
