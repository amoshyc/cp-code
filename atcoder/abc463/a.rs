#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        x: usize,
        y: usize
    }

    if x * 9 == y * 16 {
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
