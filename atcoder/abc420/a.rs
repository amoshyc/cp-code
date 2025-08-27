#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        x: usize, y: usize,
    }
    let mut ans = x + y;
    if ans >= 13 {
        ans -= 12;
    }
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
