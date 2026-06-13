#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut m: usize,
    }

    let mut cnt = 0;
    while m != 0 {
        m = n % m;
        cnt += 1;
    }

    println!("{cnt}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
