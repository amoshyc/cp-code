#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", a * 12 + b);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
