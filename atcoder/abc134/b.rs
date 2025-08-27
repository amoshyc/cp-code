#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: i64, d: i64
    }
    // ceil(n / (2 * d + 1))
    let ans = (n + (2 * d)) / (2 * d + 1);
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
