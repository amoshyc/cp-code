#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        x: u64, y: u64
    }

    let f = |x: u64| -> u64 {
        x.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    };

    let mut arr = vec![0; 11];
    arr[1] = x;
    arr[2] = y;
    for i in 3..=10 {
        arr[i] = f(arr[i - 1] + arr[i - 2]);
    }
    println!("{}", arr[10]);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
