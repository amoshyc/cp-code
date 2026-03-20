#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        let s = i.to_string().chars().collect::<Vec<char>>();
        let s = s.iter().map(|&c| c as usize - '0' as usize).sum::<usize>();
        if s == k {
            ans += 1;
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
