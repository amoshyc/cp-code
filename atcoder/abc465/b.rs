#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        x: usize,
        y: usize,
        l: usize,
        r: usize,
        a: usize,
        b: usize,
    }

    let mut ans = 0;
    for i in a..b {
        if l <= i && i < r {
            ans += x;
        } else {
            ans += y;
        }
    }
    println!("{}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
