#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        arr_a: [usize; n],
        arr_b: [usize; m],
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((arr_a[i], i));
    }

    for i in 0..m {
        if let Some(&(x, i)) = set.range((arr_b[i], 0)..=(arr_b[i], n)).next() {
            set.remove(&(x, i));
        }
    }

    let ans: Vec<usize> = set.iter().map(|&(x, i)| x).collect();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
