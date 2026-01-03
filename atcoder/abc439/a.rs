#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", (1 << n) - 2 * n);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
