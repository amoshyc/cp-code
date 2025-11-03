#![allow(unused)]

use std::collections::HashSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        arr: [Chars; n],
    }

    let mut set = HashSet::new();
    for r in (0..n).take_while(|r| r + m <= n) {
        for c in (0..n).take_while(|c| c + m <= n) {
            let mut patch = vec![];
            for i in 0..m {
                for j in 0..m {
                    patch.push(arr[r + i][c + j]);
                }
            }
            set.insert(patch);
        }
    }

    println!("{}", set.len());
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
