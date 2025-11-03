#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a == b || b == c || a == c {
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
