#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: i64
    }

    let ans = (1..=n)
        .map(|i| (-1i64).pow(i as u32) * i.pow(3))
        .sum::<i64>();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
