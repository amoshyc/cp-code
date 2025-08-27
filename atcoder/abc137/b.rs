#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: isize,
        x: isize,
    }

    let ans: Vec<isize> = (x - (k - 1)..=x + (k - 1)).collect();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
