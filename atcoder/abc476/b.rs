#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    if (0..n).filter(|&i| t[i] != '*').all(|i| t[i] == s[i]) {
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
