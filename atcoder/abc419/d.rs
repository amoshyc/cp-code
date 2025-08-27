#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        segs: [(Usize1, Usize1); m]
    }

    let mut diff = vec![0; n + 1];
    for &(l, r) in &segs {
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut cnt = vec![diff[0]];
    for i in 1..n {
        cnt.push(cnt[i - 1] + diff[i]);
    }

    let ans = (0..n)
        .map(|i| if cnt[i] % 2 == 0 { s[i] } else { t[i] })
        .collect::<Vec<char>>();

    println!("{}", join(&ans, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
