#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        a: usize, b: usize,
    }

    if (a + b) % 2 == 0 {
        println!("{}", (a + b) / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
