#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: usize,
        d: usize,
    }

    if d >= a {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
