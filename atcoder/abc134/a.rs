#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        r: i64
    }
    println!("{}", 3 * r * r);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
