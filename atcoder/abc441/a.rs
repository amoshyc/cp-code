#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    if (p..p + 100).contains(&x) && (q..q + 100).contains(&y) {
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
