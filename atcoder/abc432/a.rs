#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        mut arr: [i64; 3],
    }
    arr.sort();
    arr.reverse();

    println!("{}", join(&arr, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
