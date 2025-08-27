#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [(i64, i64); n],
    }

    let rs = arr.iter().map(|&(r, c)| r).collect::<Vec<_>>();
    let cs = arr.iter().map(|&(r, c)| c).collect::<Vec<_>>();
    let d1 = rs.iter().max().unwrap() - rs.iter().min().unwrap();
    let d2 = cs.iter().max().unwrap() - cs.iter().min().unwrap();
    println!("{}", (d1.max(d2) + 1) / 2);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
