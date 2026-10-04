#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }

    xy.sort_by_key(|&(x, y)| (x, y));

    let mut ys = BTreeSet::new();
    let mut outside = vec![false; n];
    for (i, &(x, y)) in xy.iter().enumerate() {
        if let Some(_) = ys.range(..y).next() {
            outside[i] = true;
        }
        ys.insert(y);
    }

    let ans = outside.iter().filter(|&x| !x).count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
