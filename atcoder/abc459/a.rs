#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: usize,
    }

    let mut ans = "HelloWorld".chars().collect::<Vec<char>>();
    ans.remove(x - 1);
    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
