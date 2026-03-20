#![allow(unused)]

use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        d: f64,
    }

    println!("{:.7}", PI * d * d / 4.0);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
