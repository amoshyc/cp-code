#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{s}s");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
