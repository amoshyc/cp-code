#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize
    }

    let ans = (1..=n)
        .filter(|&i| i.to_string().chars().collect::<Vec<char>>().len() % 2 == 1)
        .count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
