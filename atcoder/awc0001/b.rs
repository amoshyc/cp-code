#![allow(unused)]

use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        p: [usize; n],
    }

    if let Some(&max) = p.iter().filter(|&x| l <= *x && *x <= r).max() {
        let ans = (0..n).filter(|&i| p[i] == max).next().unwrap();
        println!("{}", ans + 1);
    } else {
        println!("-1");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
