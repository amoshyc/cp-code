#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    if a + b == 9 || a - b == 9 || a * b == 9 || a == 9 * b {
        println!("Nine");
    } else {
        println!("Nein");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
