#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: i64,
        w: i64,
    }

    // w / (h / 100) / (h / 100) >= 25
    if 100 * 100 * w >= 25 * h * h {
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
