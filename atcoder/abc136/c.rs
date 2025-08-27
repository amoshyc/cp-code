#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut suff = vec![0; n];
    suff[n - 1] = arr[n - 1];
    for i in (0..(n - 1)).rev() {
        suff[i] = suff[i + 1].min(arr[i]);
    }

    if (0..n - 1).all(|i| arr[i] <= suff[i + 1] + 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
