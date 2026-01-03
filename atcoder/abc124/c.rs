#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let t1: Vec<char> = (0..n).map(|i| if i % 2 == 0 { '0' } else { '1' }).collect();
    let t2: Vec<char> = (0..n).map(|i| if i % 2 == 0 { '1' } else { '0' }).collect();
    let c1 = (0..n).filter(|&i| t1[i] != s[i]).count();
    let c2 = (0..n).filter(|&i| t2[i] != s[i]).count();

    println!("{}", c1.min(c2));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
