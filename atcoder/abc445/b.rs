#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    let max = s.iter().map(|s| s.len()).max().unwrap();
    for s in s {
        let p = (max - s.len()) / 2;
        println!(
            "{}{}{}",
            join(&vec!['.'; p], ""),
            join(&s, ""),
            join(&vec!['.'; p], "")
        );
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
