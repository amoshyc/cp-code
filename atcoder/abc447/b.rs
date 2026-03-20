#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut freq = vec![0; 26];
    for &c in &s {
        freq[c as usize - 'a' as usize] += 1;
    }

    let max = *freq.iter().max().unwrap();
    let res = s
        .iter()
        .filter(|&&c| freq[c as usize - 'a' as usize] != max)
        .collect::<Vec<_>>();
    println!("{}", join(&res, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
