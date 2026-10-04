#![allow(unused)]

use std::collections::HashMap;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr: [Chars; n],
    }

    let mut cnt = HashMap::new();
    for s in &arr {
        let s: Vec<char> = s.iter().map(|&c| c.to_ascii_lowercase()).collect();
        *cnt.entry(s).or_insert(0) += 1;
    }

    let max = cnt.values().max().unwrap();
    println!("{max}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
