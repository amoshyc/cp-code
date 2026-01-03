#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    // w * 1000 < nb
    // n > 1000w/b
    // n >= floor_div(1000w, b) + 1
    let n = 1000 * w / b + 1;
    println!("{n}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
