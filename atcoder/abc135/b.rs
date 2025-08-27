#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut s = arr.clone();
    s.sort();

    let cnt = (0..n).filter(|&i| arr[i] != s[i]).count();
    if cnt <= 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
