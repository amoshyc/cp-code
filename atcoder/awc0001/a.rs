#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        k: i64,
    }

    println!("{}", k + 1);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
