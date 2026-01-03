#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        f: Usize1,
    }

    // f, f + 7, ..., f + 7k
    // f + 7k >= d
    // k >= ceil((d - f) / 7)

    let k = (d - f + 6) / 7;
    let ans = (f + 7 * k) - d + 1;
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
