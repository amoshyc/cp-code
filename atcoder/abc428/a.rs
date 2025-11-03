#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }

    let n = x / (a + b);
    let r = x % (a + b);
    let mut ans = n * a * s;
    ans += r.min(a) * s;
    println!("{}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
