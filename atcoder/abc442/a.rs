#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let ans = s.iter().filter(|&c| *c == 'i' || *c == 'j').count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
