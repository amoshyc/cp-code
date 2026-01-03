#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    println!("{}", (a - b).max(0));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
