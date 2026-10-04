#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut who = vec![vec![]; n];
    for i in 0..n {
        input! {
            k: usize,
            arr: [Usize1; k],
        }
        for x in arr {
            who[x].push(i + 1);
        }
    }

    for i in 0..n {
        println!("{} {}", who[i].len(), join(&who[i], " "));
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
