#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
        x: usize,
        arr: [isize; t + 1],
    }

    println!("{} {}", 0, arr[0]);
    let mut last = arr[0];
    for i in 1..=t {
        if arr[i].abs_diff(last) >= x {
            last = arr[i];
            println!("{} {}", i, arr[i]);
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
