#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let pos_b = (0..s.len())
        .filter(|&i| s[i] == 'B')
        .collect::<Vec<usize>>();

    // to ABABAB...
    let ans1 = pos_b
        .iter()
        .enumerate()
        .map(|(i, x)| x.abs_diff(2 * i + 1) as u64)
        .sum::<u64>();

    // to BABABA...
    let ans2 = pos_b
        .iter()
        .enumerate()
        .map(|(i, x)| x.abs_diff(2 * i) as u64)
        .sum::<u64>();


    println!("{}", ans1.min(ans2));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
