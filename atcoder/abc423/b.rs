#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let i = arr.iter().position(|x| *x == 1);
    let j = arr.iter().rposition(|x| *x == 1);
    if let Some((i, j)) = i.zip(j) {
        if i <= j {
            println!("{}", j - i);
        } else {
            println!("0");
        }
    } else {
        println!("0");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
