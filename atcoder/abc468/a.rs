#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let ans = (1..(n - 1))
        .filter(|&i| arr[i - 1] < arr[i] && arr[i] > arr[i + 1])
        .count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
