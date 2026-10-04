#![allow(unused)]

use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: Chars,
    }

    let pos = (0..n).filter(|&i| s[i] == 'G').collect::<BTreeSet<_>>();

    let ans = (0..n)
        .filter(|&i| s[i] == '.')
        .filter(|&i| {
            if let Some(_) = pos.range(i.saturating_sub(d)..=(i + d)).next() {
                false
            } else {
                true
            }
        })
        .count();

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
