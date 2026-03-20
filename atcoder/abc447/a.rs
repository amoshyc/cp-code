#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n < 2 * m - 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
