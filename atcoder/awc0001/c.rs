#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut arr: [i64; n],
    }

    arr.sort();
    arr.reverse();
    let ans = arr[k..].iter().sum::<i64>();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
