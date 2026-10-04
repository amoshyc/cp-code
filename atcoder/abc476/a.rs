#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        mut s: Chars,
    }

    let x = s[s.len() - 1];
    if x == 'e' {
        s.push('r');
    } else {
        s.push('e');
        s.push('r');
    }

    println!("{}", join(&s, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
