#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    println!("{}", r - l + 1);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
