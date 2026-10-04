#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        abs: [(i64, i64, Chars); n],
    }

    let mut x = 0;
    for (a, b, s) in &abs {
        if s[0] == 't' {
            x += *b - *a;
        }
    }

    let mut y = 0;
    for (a, b, s) in &abs {
        y += *b - *a;
    }

    println!("{}", y - x);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
