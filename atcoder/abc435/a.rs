#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    println!("{}", n * (n + 1) / 2);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
