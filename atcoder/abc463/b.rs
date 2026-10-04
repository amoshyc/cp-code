#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        c: char,
        arr: [Chars; n],
    }

    let i = c as usize - 'A' as usize;
    if arr.iter().any(|s| s[i] == 'o') {
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
