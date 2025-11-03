#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        events: [(Usize1, Usize1); k],
    }

    let mut cnt = vec![0; n];
    let mut last = vec![0; n];
    for (i, &(a, b)) in events.iter().enumerate() {
        cnt[a] += 1;
        last[a] = i;
    }

    let mut ans = (0..n).filter(|&i| cnt[i] == m).collect::<Vec<usize>>();
    ans.sort_by_key(|&i| last[i]);
    ans = ans.iter().map(|i| i + 1).collect();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
