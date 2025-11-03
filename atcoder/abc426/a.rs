#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
        y: Chars,
    }

    let s = ['O', 'S', 'L'];
    let x = s.iter().position(|c| *c == x[0]).unwrap();
    let y = s.iter().position(|c| *c == y[0]).unwrap();
    if x >= y {
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
