#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = (a + a - 1).max(b + b - 1).max(a + b);
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
