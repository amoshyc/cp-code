#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        s: String,
    }

    let ans = match s.as_str() {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    };
    println!("{}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
