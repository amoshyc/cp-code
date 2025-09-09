#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        s: Chars,
    }

    let mut a = s[0] as usize - '0' as usize;
    let mut b = s[2] as usize - '0' as usize;

    b += 1;
    if b == 9 {
        b = 1;
        a += 1;
    }
    println!("{}-{}", a, b);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
