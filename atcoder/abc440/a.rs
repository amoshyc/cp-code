#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: u32,
    }

    println!("{}", x * 2i64.pow(y));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
