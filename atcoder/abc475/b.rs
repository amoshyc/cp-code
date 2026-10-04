#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut cnt = vec![0, 0, 0];
    for &x in &arr {
        let pay = (x + 999) / 1000 * 1000;
        let change = pay - x;

        cnt[0] += change / 1 % 10;
        cnt[1] += change / 10 % 10;
        cnt[2] += change / 100 % 10;
    }

    println!("{}", join(&cnt, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
