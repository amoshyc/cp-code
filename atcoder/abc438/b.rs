#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let s: Vec<_> = s.iter().map(|&c| c as i32 - '0' as i32).collect();
    let t: Vec<_> = t.iter().map(|&c| c as i32 - '0' as i32).collect();

    let ans = s
        .windows(m)
        .map(|w| {
            w.iter()
                .zip(t.iter())
                .map(|(&a, &b)| (a - b).rem_euclid(10))
                .sum::<i32>()
        })
        .min()
        .unwrap();

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
