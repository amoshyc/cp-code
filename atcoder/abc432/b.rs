#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut t: Vec<_> = s.iter().filter(|&c| *c != '0').collect();
    t.sort();

    let mut ans = vec![];
    ans.push(t[0]);
    for _ in 0..(s.len() - t.len()) {
        ans.push(&'0');
    }
    ans.extend(t[1..].to_vec());

    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
